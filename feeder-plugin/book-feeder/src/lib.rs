//! `book-feeder` library surface — exposes the Gutenberg plugin so integration
//! tests (and, later, other binaries) can construct it. The binary
//! (`main.rs`) is a thin wrapper over [`meta_feeder_sdk::serve_feeders`].

pub mod gutenberg;
