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

use std::{fmt, error};

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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::XmlWriteError(ref err) => write!(f, "sitemap error: {}", err),
            Error::Invalid(s) => write!(f, "sitemap error: {}", s),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::XmlWriteError(ref err) => Some(err),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        // This method is soft-deprecated.
        "sitemap error"
    }
}

