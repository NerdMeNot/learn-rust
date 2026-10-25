//! A small file indexer: walk a directory and count the lines, words and bytes of every
//! file in it, a bounded number of files at a time.
pub mod index;
pub mod pipeline;
pub mod report;
pub mod walk;
