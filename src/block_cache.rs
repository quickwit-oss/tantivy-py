//! A process-wide, size-bounded LRU cache of fixed-size blocks read from an
//! object store.
//!
//! Search does many small random reads (term dictionary blocks, posting
//! lists, doc store blocks, fast field columns). Against an object store each
//! uncached read is a network round trip, so reads are rounded out to whole
//! blocks and the blocks are kept. Segment files are immutable once written,
//! so a cached block never goes stale; each file is keyed by its location and
//! ETag (or size, if the store has no ETag) in case a name is ever reused.
//!
//! The cache holds at most `TANTIVY_OBJECT_STORE_CACHE_BYTES` bytes (default
//! 256 MiB; `0` disables it), shared by every object store index in the
//! process.

use std::{
    ops::Range,
    sync::{Arc, Mutex, OnceLock},
};

use bytes::Bytes;
use lru::LruCache;

/// Reads are rounded out to multiples of this. A GET costs about the same
/// round trip whether it returns 1 KB or this much.
pub(crate) const BLOCK_SIZE: usize = 128 * 1024;

const DEFAULT_CAPACITY: usize = 256 * 1024 * 1024;
const CAPACITY_ENV: &str = "TANTIVY_OBJECT_STORE_CACHE_BYTES";

/// Identifies one immutable file: its location plus its ETag or size.
pub(crate) type FileKey = Arc<str>;

pub(crate) struct BlockCache {
    capacity: usize,
    state: Mutex<State>,
}

struct State {
    blocks: LruCache<(FileKey, usize), Bytes>,
    used: usize,
}

impl BlockCache {
    pub(crate) fn new(capacity: usize) -> Self {
        BlockCache {
            capacity,
            state: Mutex::new(State {
                blocks: LruCache::unbounded(),
                used: 0,
            }),
        }
    }

    /// The process-wide cache, sized from the environment on first use.
    pub(crate) fn global() -> &'static BlockCache {
        static CACHE: OnceLock<BlockCache> = OnceLock::new();
        CACHE.get_or_init(|| {
            let capacity = match std::env::var(CAPACITY_ENV) {
                Ok(value) => value.trim().parse().unwrap_or_else(|_| {
                    eprintln!(
                        "tantivy: ignoring invalid {CAPACITY_ENV}={value:?}; \
                         expected a number of bytes"
                    );
                    DEFAULT_CAPACITY
                }),
                Err(_) => DEFAULT_CAPACITY,
            };
            BlockCache::new(capacity)
        })
    }

    pub(crate) fn is_enabled(&self) -> bool {
        self.capacity > 0
    }

    pub(crate) fn get(&self, file: &FileKey, block: usize) -> Option<Bytes> {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.blocks.get(&(file.clone(), block)).cloned()
    }

    pub(crate) fn insert(&self, file: &FileKey, block: usize, data: Bytes) {
        if data.len() > self.capacity {
            return;
        }
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(old) = state.blocks.put((file.clone(), block), data.clone())
        {
            state.used -= old.len();
        }
        state.used += data.len();
        while state.used > self.capacity {
            match state.blocks.pop_lru() {
                Some((_, evicted)) => state.used -= evicted.len(),
                None => break,
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn used(&self) -> usize {
        self.state.lock().unwrap().used
    }
}

/// The block indices that cover `range`.
pub(crate) fn blocks_for(range: &Range<usize>) -> Range<usize> {
    range.start / BLOCK_SIZE..range.end.div_ceil(BLOCK_SIZE)
}

/// The byte range of block `block` in a file of `file_len` bytes.
pub(crate) fn block_range(block: usize, file_len: usize) -> Range<usize> {
    let start = block * BLOCK_SIZE;
    start..(start + BLOCK_SIZE).min(file_len)
}

/// Groups sorted block indices into runs of consecutive blocks, so each run
/// can be fetched with one request.
pub(crate) fn runs(blocks: &[usize]) -> Vec<Range<usize>> {
    let mut runs: Vec<Range<usize>> = Vec::new();
    for &block in blocks {
        match runs.last_mut() {
            Some(run) if run.end == block => run.end += 1,
            _ => runs.push(block..block + 1),
        }
    }
    runs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evicts_least_recently_used_beyond_capacity() {
        let cache = BlockCache::new(250);
        let file: FileKey = "f".into();
        cache.insert(&file, 0, Bytes::from(vec![0; 100]));
        cache.insert(&file, 1, Bytes::from(vec![1; 100]));
        assert!(cache.get(&file, 0).is_some()); // 0 is now most recent
        cache.insert(&file, 2, Bytes::from(vec![2; 100]));
        assert!(cache.get(&file, 1).is_none());
        assert!(cache.get(&file, 0).is_some());
        assert!(cache.get(&file, 2).is_some());
        assert_eq!(cache.used(), 200);
    }

    #[test]
    fn skips_blocks_larger_than_the_cache() {
        let cache = BlockCache::new(10);
        let file: FileKey = "f".into();
        cache.insert(&file, 0, Bytes::from(vec![0; 11]));
        assert!(cache.get(&file, 0).is_none());
        assert_eq!(cache.used(), 0);
    }

    #[test]
    fn block_math() {
        assert_eq!(blocks_for(&(0..1)), 0..1);
        assert_eq!(blocks_for(&(BLOCK_SIZE - 1..BLOCK_SIZE + 1)), 0..2);
        assert_eq!(blocks_for(&(BLOCK_SIZE..2 * BLOCK_SIZE)), 1..2);
        assert_eq!(
            block_range(1, BLOCK_SIZE + 10),
            BLOCK_SIZE..BLOCK_SIZE + 10
        );
        assert_eq!(runs(&[1, 2, 3, 5, 7, 8]), vec![1..4, 5..6, 7..9]);
        assert!(runs(&[]).is_empty());
    }
}
