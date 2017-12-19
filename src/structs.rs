//! Contains structures for working with sitemap.
use url::Url;
use url;
use std::convert::From;
use chrono_utils;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono_utils::parser::parse_w3c_datetime;
use std::error::Error;
use std::fmt;
use std::num;

/// Url entry. Contains url location, modification time,
/// priority, update frequency.
#[derive(Clone,Debug)]
pub struct UrlEntry {
    /// URL of the page.
    pub loc: Location,
    /// The date of last modification of the file.
    pub lastmod: LastMod,
    /// How frequently the page is likely to change.
    pub changefreq: ChangeFreq,
    /// The priority of this URL relative to other URLs on the site.
    pub priority: Priority,
}
pub struct UrlEntryBuilder {
    url_entry: UrlEntry,
}

impl UrlEntryBuilder {
    pub fn loc(mut self, url: String) -> UrlEntryBuilder {
        self.url_entry.loc = Location::from(url);
        self
    }
    pub fn lastmod(mut self, date: DateTime<FixedOffset>) -> UrlEntryBuilder {
        self.url_entry.lastmod = LastMod::DateTime(date);
        self
    }
    pub fn changefreq(mut self, changefreq: ChangeFreq) -> UrlEntryBuilder {
        self.url_entry.changefreq = changefreq;
        self
    }
    pub fn priority(mut self, val: f32) -> UrlEntryBuilder {
        self.url_entry.priority = Priority::Value(val);
        self
    }
    pub fn build(self) -> UrlEntry {
        // TODO: add check for at least the name.
        self.url_entry
    }
}

impl UrlEntry {
    /// Creates a new empty `UrlEntry`.
    pub fn new() -> UrlEntry {
        UrlEntry {
            loc: Location::None,
            lastmod: LastMod::None,
            changefreq: ChangeFreq::None,
            priority: Priority::None,
        }
    }
    pub fn builder() -> UrlEntryBuilder {
        UrlEntryBuilder { url_entry: UrlEntry::new() }
    }
}

pub struct SiteMapEntryBuilder {
    sitemap_entry: SiteMapEntry,
}

impl SiteMapEntryBuilder {
    pub fn loc(mut self, url: String) -> SiteMapEntryBuilder {
        self.sitemap_entry.loc = Location::from(url);
        self
    }

    pub fn lastmod(mut self, date: DateTime<FixedOffset>) -> SiteMapEntryBuilder {
        self.sitemap_entry.lastmod = LastMod::DateTime(date);
        self
    }

    pub fn build(self) -> SiteMapEntry {
        // TODO: add check for at least the name.
        self.sitemap_entry
    }
}

/// Sitemap entry. Contains url location and modification time.
#[derive(Clone,Debug)]
pub struct SiteMapEntry {
    /// URL of the sitemap.
    pub loc: Location,
    /// The date of last modification of the file.
    pub lastmod: LastMod,
}
impl SiteMapEntry {
    /// Creates a new empty `SiteMapEntry`.
    pub fn new() -> SiteMapEntry {
        SiteMapEntry {
            loc: Location::None,
            lastmod: LastMod::None,
        }
    }

    pub fn builder() -> SiteMapEntryBuilder {
        SiteMapEntryBuilder { sitemap_entry: SiteMapEntry::new() }
    }
}

/// Url location.
#[derive(Debug,Clone)]
pub enum Location {
    /// No value.
    None,
    /// Url
    Url(Url),
    /// Url parse error.
    Err(url::ParseError),
}
impl Location {
    /// Returns url if present.
    pub fn get_url(&self) -> Option<Url> {
        match *self {
            Location::Url(ref url) => {
                return Some(url.clone());
            }
            _ => {
                return None;
            }
        }
    }
}
impl From<String> for Location {
    /// Parses Url from string.
    fn from(url: String) -> Self {
        match Url::parse(&url) {
            Ok(url) => {
                return Location::Url(url);
            }
            Err(error) => {
                return Location::Err(error);
            }
        }
    }
}
/// The date of last modification of the resource.
#[derive(Debug,Clone)]
pub enum LastMod {
    /// No value.
    None,
    /// Modification time
    DateTime(DateTime<FixedOffset>),
    /// Parse error
    Err(chrono_utils::parser::error::ParseError),
}
impl LastMod {
    /// Returns modification time if present.
    pub fn get_time(&self) -> Option<DateTime<FixedOffset>> {
        match *self {
            LastMod::DateTime(ref time) => {
                return Some(time.clone());
            }
            _ => {
                return None;
            }
        }
    }
}
impl From<String> for LastMod {
    fn from(time: String) -> Self {
        match parse_w3c_datetime(&time) {
            Ok(time) => {
                return LastMod::DateTime(time);
            }
            Err(error) => {
                return LastMod::Err(error);
            }			
        }
    }
}
/// Error parsing URL Priority.
#[derive(PartialEq,Debug,Clone)]
pub struct ChangeFreqParseError {
    /// Error description
    pub description: String,
}
impl ChangeFreqParseError {
    /// Creates new error.
    pub fn new(description: String) -> ChangeFreqParseError {
        ChangeFreqParseError { description: description }
    }
}
impl fmt::Display for ChangeFreqParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = try!(write!(f, "Not recognezed string '{}'", self.description));
        return Ok(());
    }
}
impl Error for ChangeFreqParseError {
    fn description(&self) -> &str {
        return "Not recognezed string";
    }
}
/// How frequently the page is likely to change.
#[derive(PartialEq,Debug,Clone)]
pub enum ChangeFreq {
    /// No value.
    None,
    /// Document that change each time.
    Always,
    /// Document that change each hour.
    Hourly,
    /// Document that change each day.
    Daily,
    /// Document that change each week.
    Weekly,
    /// Document that change each month.
    Monthly,
    /// Document that change each year.
    Yearly,
    /// Archived URL.
    Never,
    /// Parse error.
    Err(ChangeFreqParseError),
}
impl ChangeFreq {
    pub fn as_str(&self) -> &str {
        match *self {
            ChangeFreq::None => "",
            ChangeFreq::Always => "always",
            ChangeFreq::Hourly => "hourly",
            ChangeFreq::Daily => "daily",
            ChangeFreq::Weekly => "weekly",
            ChangeFreq::Monthly => "monthly",
            ChangeFreq::Yearly => "yearly",
            ChangeFreq::Never => "never",
            ChangeFreq::Err(_) => "",
        }

    }
}
impl From<String> for ChangeFreq {
    fn from(time: String) -> Self {
        let lowercase_time = time.to_lowercase();
        match lowercase_time.as_ref() {
            "always" => {
                return ChangeFreq::Always;
            }
            "hourly" => {
                return ChangeFreq::Hourly;
            }
            "daily" => {
                return ChangeFreq::Daily;
            }
            "weekly" => {
                return ChangeFreq::Weekly;
            }
            "monthly" => {
                return ChangeFreq::Monthly;
            }
            "yearly" => {
                return ChangeFreq::Yearly;
            }
            "never" => {
                return ChangeFreq::Never;
            }
            _ => {
                return ChangeFreq::Err(ChangeFreqParseError::new(time));
            }
        }
    }
}

/// The priority of this URL relative to other URLs on the site.
#[derive(Debug,Clone)]
pub enum Priority {
    /// No value.
    None,
    /// Priority
    Value(f32),
    /// Parse error.
    Err(num::ParseFloatError),
    /// Error: priority lesser than zero.
    ErrValueLesserZero(f32),
    /// Error: priority greater than one.
    ErrValueGreaterOne(f32),
}
impl Priority {
    /// Returns priority if present.
    pub fn get_priority(&self) -> Option<f32> {
        match *self {
            Priority::Value(value) => {
                return Some(value);
            }
            _ => {
                return None;
            }
        }
    }
}
impl From<String> for Priority {
    fn from(priority: String) -> Self {
        let value = priority.parse::<f32>();
        match value {
            Ok(value) => {
                if value > 1.0 {
                    return Priority::ErrValueGreaterOne(value);
                } else if value < 0.0 {
                    return Priority::ErrValueLesserZero(value);
                } else {
                    return Priority::Value(value);
                }
            }
            Err(error) => {
                return Priority::Err(error);
            }
        }
    }
}
