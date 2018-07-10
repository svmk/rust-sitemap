use std::{fmt, error};
use xml;
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

