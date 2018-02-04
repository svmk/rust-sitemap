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

/// Sitemap errors
#[derive(Debug)]
pub enum Error {
	/// Unable to write data into writer
    XmlWriteError(xml::writer::Error),
    /// Error
    Invalid(String),
}

impl From<xml::writer::Error> for Error {
    fn from(err: xml::writer::Error) -> Error {
        Error::XmlWriteError(err)
    }
}
