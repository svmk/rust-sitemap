//! This crate provides Sitemap parser and writer.
//! # Examples
//!
//! [Reading sitemap](reader/index.html#examples).
//! [Writing sitemap](writer/index.html#examples).
extern crate xml;
extern crate url;
extern crate chrono_utils;
extern crate chrono;
pub mod structs;
pub mod reader;
pub mod writer;
mod errors;
pub use errors::Error;