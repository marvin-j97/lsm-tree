use crate::{
    block_cache::BlockCache, journal::Journal, levels::Levels, memtable::MemTable,
    stop_signal::StopSignal, Config,
};
use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicU32, AtomicU64},
        Arc, RwLock,
    },
};
use std_semaphore::Semaphore;

pub struct TreeInner {
    /// Tree configuration
    pub(crate) config: Config,

    /// Next sequence number (last sequence number (LSN) + 1)
    pub(crate) next_lsn: AtomicU64,

    // TODO: move into memtable
    /// Approximate active memtable size
    /// If this grows to large, a flush is triggered
    pub(crate) approx_active_memtable_size: AtomicU32,

    pub(crate) active_memtable: Arc<RwLock<MemTable>>,

    /// Journal aka Commit log aka Write-ahead log (WAL)
    pub(crate) journal: Arc<Journal>,

    /// Memtables that are being flushed
    pub(crate) immutable_memtables: Arc<RwLock<BTreeMap<Arc<str>, Arc<MemTable>>>>,

    /// Tree levels that contain segments
    pub(crate) levels: Arc<RwLock<Levels>>,

    /// Concurrent block cache
    pub(crate) block_cache: Arc<BlockCache>,

    /// Semaphore to limit flush threads
    pub(crate) flush_semaphore: Arc<Semaphore>,

    /// Semaphore to notify compaction threads
    pub(crate) compaction_semaphore: Arc<Semaphore>,

    /// Keeps track of open snapshots
    pub(crate) open_snapshots: Arc<AtomicU32>,

    /// Notifies compaction threads that the tree is dropping
    pub(crate) stop_signal: StopSignal,
}

impl Drop for TreeInner {
    fn drop(&mut self) {
        log::debug!("Dropping TreeInner");

        log::debug!("Sending stop signal to threads");
        self.stop_signal.send();

        log::debug!("Trying to flush journal");
        if let Err(error) = self.journal.flush() {
            log::warn!("Failed to flush journal: {:?}", error);
        }

        // TODO: spin lock until thread_count reaches 0
    }
}
