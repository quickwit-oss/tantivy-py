//! tantivy's directory locks (the writer lock and the meta lock) on an
//! object store.
//!
//! Object stores have no file locks, so a lock is an object created with a
//! conditional PUT (`If-None-Match: *`) that fails if it already exists.
//! Unlike a filesystem lock, the operating system does not release it when
//! the holder dies, so:
//!
//! - While a lock is held, a background task rewrites it every
//!   `heartbeat` interval, with a conditional update on its own ETag.
//! - A lock that has not been rewritten for `stale_after` is treated as
//!   left behind by a crashed process. A new holder claims it with a
//!   conditional update against the stale ETag, so if two processes race to
//!   claim it, exactly one wins.
//! - A holder whose heartbeat finds the lock changed underneath it (it was
//!   claimed after the holder stalled for longer than `stale_after`) stops
//!   heartbeating, logs that it lost the lock, and leaves it alone on release.
//!
//! Staleness compares the store's `last_modified` time with this machine's
//! clock, so `stale_after` must comfortably exceed any clock skew between
//! machines; the default of 2 minutes does.

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use object_store::{
    path::Path as ObjectPath, ObjectStore, ObjectStoreExt, PutMode, PutPayload,
    PutResult, UpdateVersion,
};
use tantivy::directory::{error::LockError, DirectoryLock};

use crate::object_store_directory::{block_on, runtime, to_io_error};

const TIMEOUT_ENV: &str = "TANTIVY_OBJECT_STORE_LOCK_TIMEOUT_SECONDS";
const DEFAULT_STALE_AFTER: Duration = Duration::from_secs(120);

/// Retry schedule for a blocking lock; matches tantivy's default.
const LOCK_RETRIES: usize = 100;
const LOCK_RETRY_WAIT: Duration = Duration::from_millis(100);

#[derive(Clone, Copy, Debug)]
pub(crate) struct LockTiming {
    /// A lock not rewritten for this long may be claimed by someone else.
    pub(crate) stale_after: Duration,
    /// How often a held lock is rewritten.
    pub(crate) heartbeat: Duration,
}

impl LockTiming {
    pub(crate) fn new(stale_after: Duration) -> Self {
        // Four heartbeats per timeout, so a couple of failed or slow
        // refreshes don't let the lock go stale.
        let heartbeat = (stale_after / 4).max(Duration::from_millis(50));
        LockTiming {
            stale_after,
            heartbeat,
        }
    }

    pub(crate) fn from_env() -> pyo3::PyResult<Self> {
        let stale_after = match std::env::var(TIMEOUT_ENV) {
            Ok(value) => value
                .trim()
                .parse::<u64>()
                .ok()
                .filter(|secs| *secs > 0)
                .map(Duration::from_secs)
                .ok_or_else(|| {
                    pyo3::exceptions::PyValueError::new_err(format!(
                        "Invalid {TIMEOUT_ENV}: expected a positive number \
                         of seconds, got '{value}'"
                    ))
                })?,
            Err(_) => DEFAULT_STALE_AFTER,
        };
        Ok(LockTiming::new(stale_after))
    }
}

/// Diagnostic contents of a lock object, to help a person work out who holds
/// it. Nothing reads it back.
fn lock_payload() -> PutPayload {
    let host = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_default();
    let body = serde_json::json!({
        "holder": "tantivy-py",
        "pid": std::process::id(),
        "host": host,
        "refreshed_at": chrono::Utc::now().to_rfc3339(),
    });
    PutPayload::from(body.to_string().into_bytes())
}

fn version_of(result: PutResult) -> UpdateVersion {
    UpdateVersion {
        e_tag: result.e_tag,
        version: result.version,
    }
}

fn io_error(err: object_store::Error) -> LockError {
    LockError::IoError(Arc::new(to_io_error(err)))
}

pub(crate) fn acquire(
    store: &Arc<dyn ObjectStore>,
    location: ObjectPath,
    is_blocking: bool,
    timing: LockTiming,
) -> Result<DirectoryLock, LockError> {
    let mut retries_left = if is_blocking { LOCK_RETRIES } else { 0 };
    loop {
        let created = block_on(store.put_opts(
            &location,
            lock_payload(),
            PutMode::Create.into(),
        ));
        let held = match created {
            Ok(result) => Some(version_of(result)),
            Err(object_store::Error::AlreadyExists { .. }) => {
                match claim_if_stale(store, &location, timing)? {
                    Claim::Claimed(version) => Some(version),
                    // Deleted between our PUT and HEAD: try creating again
                    // straight away.
                    Claim::Vanished => continue,
                    Claim::Held => None,
                }
            }
            Err(e) => return Err(io_error(e)),
        };
        if let Some(version) = held {
            return Ok(DirectoryLock::from(Box::new(LockGuard::start(
                store.clone(),
                location,
                version,
                timing,
            ))));
        }
        if retries_left == 0 {
            return Err(LockError::LockBusy);
        }
        retries_left -= 1;
        thread::sleep(LOCK_RETRY_WAIT);
    }
}

enum Claim {
    Claimed(UpdateVersion),
    Held,
    Vanished,
}

/// Take over the lock at `location` if its holder has stopped refreshing it.
fn claim_if_stale(
    store: &Arc<dyn ObjectStore>,
    location: &ObjectPath,
    timing: LockTiming,
) -> Result<Claim, LockError> {
    let meta = match block_on(store.head(location)) {
        Ok(meta) => meta,
        Err(object_store::Error::NotFound { .. }) => {
            return Ok(Claim::Vanished)
        }
        Err(e) => return Err(io_error(e)),
    };
    let age = (chrono::Utc::now() - meta.last_modified)
        .to_std()
        .unwrap_or_default();
    if age < timing.stale_after {
        return Ok(Claim::Held);
    }
    let stale = UpdateVersion {
        e_tag: meta.e_tag,
        version: meta.version,
    };
    match block_on(store.put_opts(
        location,
        lock_payload(),
        PutMode::Update(stale).into(),
    )) {
        Ok(result) => {
            eprintln!(
                "tantivy: took over stale lock {location}, last refreshed \
                 {}s ago",
                age.as_secs()
            );
            Ok(Claim::Claimed(version_of(result)))
        }
        // Someone else refreshed or claimed it first.
        Err(object_store::Error::Precondition { .. }) => Ok(Claim::Held),
        Err(object_store::Error::NotFound { .. }) => Ok(Claim::Vanished),
        // A store without conditional updates cannot hand over a lock
        // safely; the stale lock has to be deleted by hand.
        Err(object_store::Error::NotImplemented { .. }) => Ok(Claim::Held),
        Err(e) => Err(io_error(e)),
    }
}

/// Keeps a lock fresh while held and deletes it on release.
struct LockGuard {
    store: Arc<dyn ObjectStore>,
    location: ObjectPath,
    version: Arc<Mutex<UpdateVersion>>,
    lost: Arc<AtomicBool>,
    heartbeat: tokio::task::JoinHandle<()>,
}

impl LockGuard {
    fn start(
        store: Arc<dyn ObjectStore>,
        location: ObjectPath,
        version: UpdateVersion,
        timing: LockTiming,
    ) -> Self {
        let version = Arc::new(Mutex::new(version));
        let lost = Arc::new(AtomicBool::new(false));
        let heartbeat = runtime().spawn(heartbeat(
            store.clone(),
            location.clone(),
            version.clone(),
            lost.clone(),
            timing.heartbeat,
        ));
        LockGuard {
            store,
            location,
            version,
            lost,
            heartbeat,
        }
    }
}

async fn heartbeat(
    store: Arc<dyn ObjectStore>,
    location: ObjectPath,
    version: Arc<Mutex<UpdateVersion>>,
    lost: Arc<AtomicBool>,
    interval: Duration,
) {
    loop {
        tokio::time::sleep(interval).await;
        let current = version.lock().unwrap_or_else(|e| e.into_inner()).clone();
        match store
            .put_opts(
                &location,
                lock_payload(),
                PutMode::Update(current).into(),
            )
            .await
        {
            Ok(result) => {
                *version.lock().unwrap_or_else(|e| e.into_inner()) =
                    version_of(result);
            }
            Err(
                object_store::Error::Precondition { .. }
                | object_store::Error::NotFound { .. },
            ) => {
                lost.store(true, Ordering::SeqCst);
                eprintln!(
                    "tantivy: lost lock {location}: another process took it \
                     over after this one did not refresh it in time"
                );
                return;
            }
            Err(object_store::Error::NotImplemented { .. }) => {
                // No conditional updates: the lock can't go stale-and-claimed
                // either, so there is nothing to keep fresh.
                return;
            }
            // Transient errors: keep the lock and try again next interval.
            Err(e) => {
                eprintln!("tantivy: failed to refresh lock {location}: {e}")
            }
        }
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        self.heartbeat.abort();
        if self.lost.load(Ordering::SeqCst) {
            return;
        }
        // Only delete the lock if it is still ours. Object stores have no
        // conditional delete, so a claim between this check and the delete
        // is possible, but it needs the lock to have gone stale while held.
        let ours = self
            .version
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .e_tag
            .clone();
        let result = block_on(async {
            if let Some(ours) = ours {
                match self.store.head(&self.location).await {
                    Ok(meta)
                        if meta.e_tag.as_deref() != Some(ours.as_str()) =>
                    {
                        return Ok(());
                    }
                    Err(object_store::Error::NotFound { .. }) => return Ok(()),
                    _ => {}
                }
            }
            match self.store.delete(&self.location).await {
                Err(object_store::Error::NotFound { .. }) => Ok(()),
                other => other,
            }
        });
        if let Err(e) = result {
            eprintln!("tantivy: failed to remove lock {}: {e}", self.location);
        }
    }
}

#[cfg(test)]
mod tests {
    use object_store::memory::InMemory;

    use super::*;

    fn store() -> Arc<dyn ObjectStore> {
        Arc::new(InMemory::new())
    }

    fn timing(ms: u64) -> LockTiming {
        LockTiming::new(Duration::from_millis(ms))
    }

    #[test]
    fn lock_is_exclusive_while_held() {
        let store = store();
        let loc = ObjectPath::from("idx/.tantivy-writer.lock");
        let held = acquire(&store, loc.clone(), false, timing(300)).unwrap();
        // The heartbeat keeps it fresh well past the stale timeout.
        thread::sleep(Duration::from_millis(700));
        assert!(matches!(
            acquire(&store, loc.clone(), false, timing(300)),
            Err(LockError::LockBusy)
        ));
        drop(held);
        assert!(block_on(store.head(&loc)).is_err(), "released lock deleted");
        acquire(&store, loc, false, timing(300)).unwrap();
    }

    #[test]
    fn stale_lock_from_a_dead_holder_is_taken_over() {
        let store = store();
        let loc = ObjectPath::from("idx/.tantivy-writer.lock");
        // A crashed holder: the object exists but nobody refreshes it.
        block_on(store.put(&loc, PutPayload::from_static(b"{}"))).unwrap();
        assert!(matches!(
            acquire(&store, loc.clone(), false, timing(200)),
            Err(LockError::LockBusy)
        ));
        thread::sleep(Duration::from_millis(250));
        let _held = acquire(&store, loc, false, timing(200)).unwrap();
    }

    #[test]
    fn holder_that_lost_its_lock_does_not_delete_the_new_one() {
        let store = store();
        let loc = ObjectPath::from("idx/.tantivy-writer.lock");
        let first = acquire(&store, loc.clone(), false, timing(200)).unwrap();
        // Someone else overwrites it (as a takeover would).
        block_on(store.put(&loc, PutPayload::from_static(b"other"))).unwrap();
        thread::sleep(Duration::from_millis(150)); // > one heartbeat
        drop(first);
        let body = block_on(async { store.get(&loc).await?.bytes().await })
            .expect("the new holder's lock must survive");
        assert_eq!(&body[..], b"other");
    }

    #[test]
    fn blocking_lock_waits_for_release() {
        let store = store();
        let loc = ObjectPath::from("idx/.tantivy-meta.lock");
        let held = acquire(&store, loc.clone(), false, timing(10_000)).unwrap();
        let releaser = thread::spawn(move || {
            thread::sleep(Duration::from_millis(300));
            drop(held);
        });
        acquire(&store, loc, true, timing(10_000)).unwrap();
        releaser.join().unwrap();
    }
}
