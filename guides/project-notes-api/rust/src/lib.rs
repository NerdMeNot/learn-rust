//! A notes API in three layers.
//!
//! * [`domain`]: what a note is and what text is allowed. No web, no database.
//! * [`storage`]: where notes live, behind the [`storage::NoteStore`] trait.
//! * [`http`]: how the outside world asks, written against that trait.
//!
//! `main.rs` is the only place that picks the real parts (SQLite, the environment).

pub mod config;
pub mod domain;
pub mod http;
pub mod storage;
