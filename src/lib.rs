//! This crate provides Sitemap parser.
//! # Examples
//!
//! [Reading sitemap](reader/index.html#examples).
extern crate xml;
extern crate url;
extern crate chrono_utils;
extern crate chrono;
pub mod structs;
pub mod reader;
pub mod writer;

#[derive(Debug)]
pub enum Error {
    XmlWriteError(xml::writer::Error),
    Invalid(String),
}

impl From<xml::writer::Error> for Error {
    fn from(err: xml::writer::Error) -> Error {
        Error::XmlWriteError(err)
    }
}
