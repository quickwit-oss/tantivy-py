//! A tantivy `Directory` backed by an object store (S3, GCS, Azure Blob, or
//! in-memory), built on the `object_store` crate.
//!
//! tantivy's `Directory` trait is synchronous and `object_store` is async, so
//! every call runs on one shared tokio runtime via `block_on`. tantivy never
//! calls a `Directory` from inside an async context, so this cannot deadlock.
//!
//! How each part of the trait maps onto an object store:
//!
//! - Reads are ranged GETs. There is no caching yet, so every
//!   `read_bytes` call is one request.
//! - Writes are buffered in memory and uploaded with one PUT when the file is
//!   terminated. Files larger than `MULTIPART_THRESHOLD` switch to a
//!   multipart upload, so memory use stays bounded for large segments.
//! - `atomic_write` (used for `meta.json`) is a single PUT, which object
//!   stores apply atomically.
//! - Locks are created with a conditional PUT (`PutMode::Create`), which
//!   fails if the lock object already exists. Unlike a filesystem lock, a lock
//!   left behind by a crashed writer is not released automatically.
//! - `watch` only sees commits made through this process. Changes made by
//!   another process are picked up with `Index.reload()`.

use std::{
    collections::HashMap,
    fmt,
    future::Future,
    io::{self, BufWriter, Write},
    ops::Range,
    path::Path,
    sync::{Arc, Mutex, OnceLock},
    thread,
    time::Duration,
};

use object_store::{
    memory::InMemory, path::Path as ObjectPath, ObjectStore, ObjectStoreExt,
    ObjectStoreScheme, PutMode, PutPayload, WriteMultipart,
};
use pyo3::{exceptions, PyResult};
use tantivy::{
    directory::{
        error::{DeleteError, LockError, OpenReadError, OpenWriteError},
        AntiCallToken, Directory, DirectoryLock, FileHandle, Lock, OwnedBytes,
        TerminatingWrite, WatchCallback, WatchCallbackList, WatchHandle,
        WritePtr,
    },
    HasLen,
};
use url::Url;

/// Files that grow past this size are uploaded with a multipart upload
/// instead of being held in memory until they are terminated.
const MULTIPART_THRESHOLD: usize = 16 * 1024 * 1024;
/// Upper bound on concurrently uploading parts for one file.
const MAX_CONCURRENT_PARTS: usize = 8;
/// Retry schedule for a blocking lock; matches tantivy's default.
const LOCK_RETRIES: usize = 100;
const LOCK_RETRY_WAIT: Duration = Duration::from_millis(100);

const META_FILEPATH: &str = "meta.json";

/// Reads larger than this are split into concurrent ranged GETs of about this
/// size. Polars' default for random-access reads.
const DOWNLOAD_CHUNK_SIZE: usize = 8 * 1024 * 1024;

fn runtime() -> &'static tokio::runtime::Runtime {
    static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .thread_name("tantivy-object-store")
            .enable_all()
            .build()
            .expect("failed to start the tokio runtime for object storage")
    })
}

pub(crate) fn block_on<F: Future>(future: F) -> F::Output {
    runtime().block_on(future)
}

/// One in-memory store shared by every `memory://` URI in the process, so an
/// index created at `memory:///a` can be reopened by a later call.
fn memory_store() -> Arc<dyn ObjectStore> {
    static STORE: OnceLock<Arc<InMemory>> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(InMemory::new())).clone()
}

fn to_io_error(err: object_store::Error) -> io::Error {
    let kind = match err {
        object_store::Error::NotFound { .. } => io::ErrorKind::NotFound,
        object_store::Error::AlreadyExists { .. } => {
            io::ErrorKind::AlreadyExists
        }
        object_store::Error::PermissionDenied { .. }
        | object_store::Error::Unauthenticated { .. } => {
            io::ErrorKind::PermissionDenied
        }
        _ => io::ErrorKind::Other,
    };
    io::Error::new(kind, err)
}

fn value_error(context: &str, err: impl fmt::Display) -> pyo3::PyErr {
    exceptions::PyValueError::new_err(format!("{context}: {err}"))
}

#[derive(Clone)]
pub(crate) struct ObjectStoreDirectory {
    store: Arc<dyn ObjectStore>,
    root: ObjectPath,
    uri: Arc<str>,
    watchers: Arc<WatchCallbackList>,
}

impl fmt::Debug for ObjectStoreDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectStoreDirectory({})", self.uri)
    }
}

impl ObjectStoreDirectory {
    /// Open the object store that `uri` names.
    ///
    /// Credentials and other settings are read from the provider's usual
    /// environment variables (`AWS_*`, `GOOGLE_*`, `AZURE_*`), then
    /// overridden by `options`. An option key the provider does not know is
    /// an error rather than being silently ignored.
    pub(crate) fn open(
        uri: &str,
        options: HashMap<String, String>,
    ) -> PyResult<Self> {
        let url = Url::parse(uri).map_err(|e| value_error("Invalid URI", e))?;
        let (scheme, path) = ObjectStoreScheme::parse(&url)
            .map_err(|e| value_error("Unsupported URI", e))?;
        let store = build_store(&scheme, uri, options)?;
        let root = ObjectPath::parse(path)
            .map_err(|e| value_error("Invalid path in URI", e))?;
        Ok(ObjectStoreDirectory {
            store,
            root,
            uri: uri.into(),
            watchers: Arc::default(),
        })
    }

    fn location(&self, path: &Path) -> io::Result<ObjectPath> {
        let relative = path.to_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("path is not valid UTF-8: {path:?}"),
            )
        })?;
        let relative = ObjectPath::parse(relative)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        Ok(self.root.parts().chain(relative.parts()).collect())
    }

    fn head(&self, path: &Path) -> Result<Option<usize>, OpenReadError> {
        let location = self
            .location(path)
            .map_err(|e| OpenReadError::wrap_io_error(e, path.to_owned()))?;
        match block_on(self.store.head(&location)) {
            Ok(meta) => Ok(Some(meta.size as usize)),
            Err(object_store::Error::NotFound { .. }) => Ok(None),
            Err(e) => Err(OpenReadError::wrap_io_error(
                to_io_error(e),
                path.to_owned(),
            )),
        }
    }
}

fn unknown_option(scheme: &str, key: &str) -> pyo3::PyErr {
    exceptions::PyValueError::new_err(format!(
        "Unknown storage option '{key}' for {scheme}"
    ))
}

fn build_store(
    scheme: &ObjectStoreScheme,
    uri: &str,
    options: HashMap<String, String>,
) -> PyResult<Arc<dyn ObjectStore>> {
    // `uri` and `options` are unused when every provider feature is off.
    let _ = (uri, &options);
    let store: Arc<dyn ObjectStore> = match scheme {
        ObjectStoreScheme::Memory => {
            if let Some(key) = options.keys().next() {
                return Err(unknown_option("memory://", key));
            }
            memory_store()
        }
        #[cfg(feature = "aws")]
        ObjectStoreScheme::AmazonS3 => {
            crate::cloud_options::build_aws(uri, options)?
        }
        #[cfg(feature = "gcp")]
        ObjectStoreScheme::GoogleCloudStorage => {
            crate::cloud_options::build_gcp(uri, options)?
        }
        #[cfg(feature = "azure")]
        ObjectStoreScheme::MicrosoftAzure => {
            crate::cloud_options::build_azure(uri, options)?
        }
        other => {
            return Err(exceptions::PyValueError::new_err(format!(
                "Object store {other:?} is not supported by this build of \
                 tantivy (compiled with: {})",
                compiled_backends()
            )))
        }
    };
    Ok(store)
}

fn compiled_backends() -> String {
    let backends: &[(&str, bool)] = &[
        ("memory", true),
        ("s3", cfg!(feature = "aws")),
        ("gcs", cfg!(feature = "gcp")),
        ("azure", cfg!(feature = "azure")),
    ];
    backends
        .iter()
        .filter(|(_, enabled)| *enabled)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join(", ")
}

impl Directory for ObjectStoreDirectory {
    fn get_file_handle(
        &self,
        path: &Path,
    ) -> Result<Arc<dyn FileHandle>, OpenReadError> {
        let len = self
            .head(path)?
            .ok_or_else(|| OpenReadError::FileDoesNotExist(path.to_owned()))?;
        let location = self
            .location(path)
            .map_err(|e| OpenReadError::wrap_io_error(e, path.to_owned()))?;
        Ok(Arc::new(ObjectStoreFileHandle {
            store: self.store.clone(),
            location,
            len,
        }))
    }

    fn delete(&self, path: &Path) -> Result<(), DeleteError> {
        let io_error = |io_error: io::Error| DeleteError::IoError {
            io_error: Arc::new(io_error),
            filepath: path.to_owned(),
        };
        let location = self.location(path).map_err(io_error)?;
        match block_on(self.store.delete(&location)) {
            Ok(()) => Ok(()),
            Err(object_store::Error::NotFound { .. }) => {
                Err(DeleteError::FileDoesNotExist(path.to_owned()))
            }
            Err(e) => Err(io_error(to_io_error(e))),
        }
    }

    fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
        Ok(self.head(path)?.is_some())
    }

    fn open_write(&self, path: &Path) -> Result<WritePtr, OpenWriteError> {
        let io_error = |e: io::Error| OpenWriteError::IoError {
            io_error: Arc::new(e),
            filepath: path.to_owned(),
        };
        let exists = self
            .head(path)
            .map_err(|e| io_error(io::Error::other(e.to_string())))?;
        if exists.is_some() {
            return Err(OpenWriteError::FileAlreadyExists(path.to_owned()));
        }
        let location = self.location(path).map_err(io_error)?;
        let writer = ObjectStoreWriter {
            store: self.store.clone(),
            location,
            state: WriterState::Buffering(Vec::new()),
        };
        Ok(BufWriter::new(Box::new(writer)))
    }

    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
        let location = self
            .location(path)
            .map_err(|e| OpenReadError::wrap_io_error(e, path.to_owned()))?;
        let result =
            block_on(async { self.store.get(&location).await?.bytes().await });
        match result {
            Ok(bytes) => Ok(bytes.to_vec()),
            Err(object_store::Error::NotFound { .. }) => {
                Err(OpenReadError::FileDoesNotExist(path.to_owned()))
            }
            Err(e) => Err(OpenReadError::wrap_io_error(
                to_io_error(e),
                path.to_owned(),
            )),
        }
    }

    fn atomic_write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        let location = self.location(path)?;
        block_on(self.store.put(&location, PutPayload::from(data.to_vec())))
            .map_err(to_io_error)?;
        if path == Path::new(META_FILEPATH) {
            // Readers in this process reload on commit, as with RamDirectory.
            drop(self.watchers.broadcast());
        }
        Ok(())
    }

    fn sync_directory(&self) -> io::Result<()> {
        // A completed PUT is already durable.
        Ok(())
    }

    fn acquire_lock(&self, lock: &Lock) -> Result<DirectoryLock, LockError> {
        let location = self
            .location(&lock.filepath)
            .map_err(|e| LockError::IoError(Arc::new(e)))?;
        let mut retries_left = if lock.is_blocking { LOCK_RETRIES } else { 0 };
        loop {
            let created = block_on(self.store.put_opts(
                &location,
                PutPayload::new(),
                PutMode::Create.into(),
            ));
            match created {
                Ok(_) => {
                    return Ok(DirectoryLock::from(Box::new(LockGuard {
                        store: self.store.clone(),
                        location,
                    })))
                }
                Err(object_store::Error::AlreadyExists { .. }) => {
                    if retries_left == 0 {
                        return Err(LockError::LockBusy);
                    }
                    retries_left -= 1;
                    thread::sleep(LOCK_RETRY_WAIT);
                }
                Err(e) => {
                    return Err(LockError::IoError(Arc::new(to_io_error(e))))
                }
            }
        }
    }

    fn watch(
        &self,
        watch_callback: WatchCallback,
    ) -> tantivy::Result<WatchHandle> {
        Ok(self.watchers.subscribe(watch_callback))
    }
}

/// Deletes the lock object when the lock is released.
struct LockGuard {
    store: Arc<dyn ObjectStore>,
    location: ObjectPath,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        if let Err(e) = block_on(self.store.delete(&self.location)) {
            eprintln!("tantivy: failed to remove lock {}: {e}", self.location);
        }
    }
}

#[derive(Debug)]
struct ObjectStoreFileHandle {
    store: Arc<dyn ObjectStore>,
    location: ObjectPath,
    len: usize,
}

impl HasLen for ObjectStoreFileHandle {
    fn len(&self) -> usize {
        self.len
    }
}

impl FileHandle for ObjectStoreFileHandle {
    fn read_bytes(&self, range: Range<usize>) -> io::Result<OwnedBytes> {
        if range.is_empty() {
            return Ok(OwnedBytes::empty());
        }
        let start = range.start;
        let mut data = vec![0u8; range.len()];
        let parts = split_range(range, DOWNLOAD_CHUNK_SIZE).map(|part| {
            let store = &self.store;
            let location = &self.location;
            async move {
                let bytes = store
                    .get_range(location, part.start as u64..part.end as u64)
                    .await?;
                Ok::<_, object_store::Error>((part, bytes))
            }
        });
        let parts = block_on(futures::future::try_join_all(parts))
            .map_err(to_io_error)?;
        for (part, bytes) in parts {
            if bytes.len() != part.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    format!(
                        "short read from {}: wanted {} bytes, got {}",
                        self.location,
                        part.len(),
                        bytes.len()
                    ),
                ));
            }
            data[part.start - start..part.end - start].copy_from_slice(&bytes);
        }
        Ok(OwnedBytes::new(data))
    }
}

/// Splits a single range into multiple smaller ranges, which can be
/// downloaded concurrently for much higher throughput.
///
/// Copied from Polars (`crates/polars-io/src/cloud/polars_object_store.rs`),
/// Copyright (c) 2025 Ritchie Vink, MIT License; see `licenses/polars-LICENSE`.
/// Changed only to take `chunk_size` directly.
fn split_range(
    range: Range<usize>,
    chunk_size: usize,
) -> impl ExactSizeIterator<Item = Range<usize>> {
    // Calculate n_parts such that we are as close as possible to the `chunk_size`.
    let n_parts = [
        (range.len().div_ceil(chunk_size)).max(1),
        (range.len() / chunk_size).max(1),
    ]
    .into_iter()
    .min_by_key(|x| (range.len() / *x).abs_diff(chunk_size))
    .unwrap();

    let chunk_size = (range.len() / n_parts).max(1);

    assert_eq!(n_parts, (range.len() / chunk_size).max(1));
    let bytes_rem = range.len() % chunk_size;

    (0..n_parts).map(move |part_no| {
        let (start, end) = if part_no == 0 {
            // Download remainder length in the first chunk since it starts downloading first.
            let end = range.start + chunk_size + bytes_rem;
            let end = if end > range.end { range.end } else { end };
            (range.start, end)
        } else {
            let start = bytes_rem + range.start + part_no * chunk_size;
            (start, start + chunk_size)
        };

        start..end
    })
}

enum WriterState {
    /// Small files stay in memory and are uploaded with one PUT.
    Buffering(Vec<u8>),
    /// Large files stream out as a multipart upload. The mutex only makes the
    /// writer `Sync`, as `WritePtr` requires; it is reached through
    /// `get_mut`, so it is never contended.
    Uploading(Mutex<WriteMultipart>),
    Terminated,
}

struct ObjectStoreWriter {
    store: Arc<dyn ObjectStore>,
    location: ObjectPath,
    state: WriterState,
}

impl ObjectStoreWriter {
    fn start_multipart(&mut self, buffered: &[u8]) -> io::Result<()> {
        let upload = block_on(self.store.put_multipart(&self.location))
            .map_err(to_io_error)?;
        let mut upload = WriteMultipart::new(upload);
        // `write` spawns part uploads onto the runtime.
        let _guard = runtime().enter();
        upload.write(buffered);
        self.state = WriterState::Uploading(Mutex::new(upload));
        Ok(())
    }
}

fn poisoned<T>(_: std::sync::PoisonError<T>) -> io::Error {
    io::Error::other("multipart upload state poisoned by an earlier panic")
}

impl Write for ObjectStoreWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match &mut self.state {
            WriterState::Buffering(buffer) => {
                buffer.extend_from_slice(buf);
                if buffer.len() >= MULTIPART_THRESHOLD {
                    let buffered = std::mem::take(buffer);
                    self.start_multipart(&buffered)?;
                }
            }
            WriterState::Uploading(upload) => {
                let upload = upload.get_mut().map_err(poisoned)?;
                block_on(upload.wait_for_capacity(MAX_CONCURRENT_PARTS))
                    .map_err(to_io_error)?;
                let _guard = runtime().enter();
                upload.write(buf);
            }
            WriterState::Terminated => {
                return Err(io::Error::other("write after terminate"));
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // Data becomes visible only when the file is terminated.
        Ok(())
    }
}

impl TerminatingWrite for ObjectStoreWriter {
    fn terminate_ref(&mut self, _: AntiCallToken) -> io::Result<()> {
        match std::mem::replace(&mut self.state, WriterState::Terminated) {
            WriterState::Buffering(buffer) => {
                block_on(self.store.put(&self.location, buffer.into()))
                    .map_err(to_io_error)?;
            }
            WriterState::Uploading(upload) => {
                let upload = upload.into_inner().map_err(poisoned)?;
                block_on(upload.finish()).map_err(to_io_error)?;
            }
            WriterState::Terminated => {}
        }
        Ok(())
    }
}

impl Drop for ObjectStoreWriter {
    fn drop(&mut self) {
        // A file dropped without being terminated must not become visible.
        if let WriterState::Uploading(upload) =
            std::mem::replace(&mut self.state, WriterState::Terminated)
        {
            if let Ok(upload) = upload.into_inner() {
                let _ = block_on(upload.abort());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn memory_dir(prefix: &str) -> ObjectStoreDirectory {
        ObjectStoreDirectory::open(prefix, HashMap::new()).unwrap()
    }

    #[test]
    fn write_then_read_back() {
        let dir = memory_dir("memory:///unit/write_then_read_back");
        let path = Path::new("seg.idx");
        let mut w = dir.open_write(path).unwrap();
        w.write_all(b"hello world").unwrap();
        w.terminate().unwrap();

        let slice = dir.open_read(path).unwrap();
        assert_eq!(slice.len(), 11);
        assert_eq!(slice.read_bytes_slice(6..11).unwrap().as_slice(), b"world");
        assert!(matches!(
            dir.open_write(path),
            Err(OpenWriteError::FileAlreadyExists(_))
        ));
    }

    #[test]
    fn large_file_uses_multipart() {
        let dir = memory_dir("memory:///unit/large_file_uses_multipart");
        let path = Path::new("big.store");
        let data: Vec<u8> = (0..MULTIPART_THRESHOLD * 2 + 123)
            .map(|i| i as u8)
            .collect();
        let mut w = dir.open_write(path).unwrap();
        for chunk in data.chunks(1 << 20) {
            w.write_all(chunk).unwrap();
        }
        w.terminate().unwrap();
        let read = dir.open_read(path).unwrap().read_bytes().unwrap();
        assert_eq!(read.as_slice(), &data[..]);
    }

    #[test]
    fn split_range_covers_range_exactly() {
        for (len, chunk) in [(1, 8), (8, 8), (9, 8), (100, 8), (65, 64)] {
            let range = 10..10 + len;
            let parts: Vec<_> = split_range(range.clone(), chunk).collect();
            assert_eq!(parts.first().unwrap().start, range.start);
            assert_eq!(parts.last().unwrap().end, range.end);
            for pair in parts.windows(2) {
                assert_eq!(pair[0].end, pair[1].start);
            }
        }
        // Polars' own expectation: 8 parts of ~chunk size, no tiny tail.
        let parts: Vec<_> = split_range(0..8 * 1000 + 3, 1000).collect();
        assert_eq!(parts.len(), 8);
        assert_eq!(parts[0], 0..1003);
    }

    #[test]
    fn large_read_is_split_and_reassembled() {
        let dir = memory_dir("memory:///unit/large_read");
        let path = Path::new("big.idx");
        let data: Vec<u8> = (0..DOWNLOAD_CHUNK_SIZE * 3 + 7)
            .map(|i| (i % 251) as u8)
            .collect();
        let mut w = dir.open_write(path).unwrap();
        w.write_all(&data).unwrap();
        w.terminate().unwrap();
        let slice = dir.open_read(path).unwrap();
        let range = 5..data.len() - 2;
        assert_eq!(
            slice.read_bytes_slice(range.clone()).unwrap().as_slice(),
            &data[range]
        );
    }

    #[test]
    fn unterminated_file_is_not_visible() {
        let dir = memory_dir("memory:///unit/unterminated");
        let path = Path::new("partial");
        let mut w = dir.open_write(path).unwrap();
        w.write_all(b"data").unwrap();
        drop(w);
        assert!(!dir.exists(path).unwrap());
    }

    #[test]
    fn lock_is_exclusive_until_released() {
        let dir = memory_dir("memory:///unit/lock");
        let lock = Lock {
            filepath: PathBuf::from(".tantivy-writer.lock"),
            is_blocking: false,
        };
        let held = dir.acquire_lock(&lock).unwrap();
        assert!(matches!(dir.acquire_lock(&lock), Err(LockError::LockBusy)));
        drop(held);
        dir.acquire_lock(&lock).unwrap();
    }

    #[test]
    fn missing_files_report_not_found() {
        let dir = memory_dir("memory:///unit/missing");
        let path = Path::new("nope");
        assert!(!dir.exists(path).unwrap());
        assert!(matches!(
            dir.atomic_read(path),
            Err(OpenReadError::FileDoesNotExist(_))
        ));
        assert!(matches!(
            dir.open_read(path),
            Err(OpenReadError::FileDoesNotExist(_))
        ));
    }

    #[test]
    fn prefixes_are_isolated() {
        let a = memory_dir("memory:///unit/iso/a");
        let b = memory_dir("memory:///unit/iso/b");
        a.atomic_write(Path::new("meta.json"), b"{}").unwrap();
        assert!(a.exists(Path::new("meta.json")).unwrap());
        assert!(!b.exists(Path::new("meta.json")).unwrap());
    }

    #[test]
    fn rejects_unknown_memory_option() {
        let mut opts = HashMap::new();
        opts.insert("region".to_string(), "x".to_string());
        assert!(ObjectStoreDirectory::open("memory:///x", opts).is_err());
    }
}
