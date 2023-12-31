//! A K.I.S.S. implementation of log-structured merge trees (LSM-trees/LSMTs).
//!
//! This crate exports a persistent `Tree` that supports a subset of the `BTreeMap` API.
//!
//! LSM-trees are an alternative to B-trees to persist a sorted list of items (e.g. a database table)
//! on disk and perform fast lookup queries.
//! Instead of updating a disk-based data structure in-place,
//! deltas (inserts and deletes) are added into an in-memory write buffer (`MemTable`).
//! To provide durability, the `MemTable` is backed by a disk-based journal.
//! When the `MemTable` exceeds a size threshold, it is flushed to a sorted file
//! on disk ("Segment" a.k.a. "`SSTable`").
//! Items can then be retrieved by checking and merging results from the `MemTable` and Segments.
//!
//! Amassing many segments on disk will degrade read performance and waste disk space usage, so segments are
//! periodically merged into larger segments in a process called "Compaction".
//! Different compaction strategies have different advantages and drawbacks, depending on your use case.
//!
//! Because maintaining an efficient structure is deferred to the compaction process, writing to an LSMT
//! is very fast (O(1) complexity).
//!
//! # Example usage
//!
//! ```
//! use lsm_tree::{Tree, Config};
//!
//! # let folder = tempfile::tempdir()?;
//! // A tree is a single physical keyspace/index/...
//! // and supports a BTreeMap-like API
//! // but all data is persisted to disk.
//! let tree = Config::new(folder).open()?;
//!
//! // Note compared to the BTreeMap API, operations return a Result<T>
//! // So you can handle I/O errors if they occur
//! tree.insert("my_key", "my_value")?;
//!
//! let item = tree.get("my_key")?;
//! assert_eq!(Some("my_value".as_bytes().into()), item);
//!
//! // Flush to definitely make sure data is persisted
//! tree.flush()?;
//!
//! // Search by prefix
//! for item in &tree.prefix("prefix") {
//!   // ...
//! }
//!
//! // Search by range
//! for item in &tree.range("a"..="z") {
//!   // ...
//! }
//!
//! // Iterators implement DoubleEndedIterator, so you can search backwards, too!
//! for item in tree.prefix("prefix").into_iter().rev() {
//!   // ...
//! }
//! #
//! # Ok::<(), lsm_tree::Error>(())
//! ```

#![doc(html_logo_url = "https://raw.githubusercontent.com/marvin-j97/lsm-tree/main/logo.png")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/marvin-j97/lsm-tree/main/logo.png")]
#![forbid(unsafe_code)]
#![deny(clippy::all, missing_docs, clippy::cargo)]
#![deny(clippy::unwrap_used)]
#![warn(clippy::pedantic, clippy::nursery)]
#![warn(clippy::expect_used)]
#![allow(clippy::missing_const_for_fn)]

mod batch;
mod block_cache;
pub mod compaction;
mod config;
mod descriptor_table;
mod disk_block;
mod either;
mod entry;
mod error;
mod file;
mod flush;
mod id;
mod journal;
mod levels;

#[doc(hidden)]
pub mod memtable;

mod merge;
mod prefix;
mod range;
mod recovery;
mod segment;
mod serde;
mod sharded;
mod snapshot;
mod stop_signal;
mod time;
mod tree;
mod tree_inner;
mod value;
mod version;

#[doc(hidden)]
pub use value::{Value, ValueType};

pub use {
    crate::serde::{DeserializeError, SerializeError},
    batch::Batch,
    block_cache::BlockCache,
    config::Config,
    entry::Entry,
    error::{Error, Result},
    journal::shard::RecoveryError as JournalRecoveryError,
    snapshot::Snapshot,
    tree::Tree,
};
