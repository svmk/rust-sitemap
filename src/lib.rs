//! This crate provides Sitemap parser and writer.
//! # Examples
//!
//! [Reading sitemap](reader/index.html#examples).
//! [Writing sitemap](writer/index.html#examples).
pub mod structs;
pub mod reader;
pub mod writer;
mod errors;
pub use errors::Error;