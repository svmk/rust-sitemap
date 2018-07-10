//! Contains structures for working with sitemap.
use url::Url;
use url;
use std::convert::From;
use std::convert::Into;
use chrono_utils;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono_utils::parser::parse_w3c_datetime;
use std::fmt;
use std::num;
use Error;

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

    /// Creates builder for `UrlEntry` structure
    pub fn builder() -> UrlEntryBuilder {
        UrlEntryBuilder { url_entry: UrlEntry::new() }
    }
}

/// Builds `UrlEntry` structure
#[derive(Clone,Debug)]
pub struct UrlEntryBuilder {
    url_entry: UrlEntry,
}

impl UrlEntryBuilder {
    /// Defines `loc` tag
    pub fn loc<S: Into<String>>(mut self, url: S) -> UrlEntryBuilder {
        let url = url.into();
        self.url_entry.loc = Location::from(url);
        return self;
    }

    /// Defines `lastmod` tag
    pub fn lastmod(mut self, date: DateTime<FixedOffset>) -> UrlEntryBuilder {
        self.url_entry.lastmod = LastMod::DateTime(date);
        return self;
    }

    /// Defines `changefreq` tag
    pub fn changefreq(mut self, changefreq: ChangeFreq) -> UrlEntryBuilder {
        self.url_entry.changefreq = changefreq;
        return self;
    }

    /// Defines `priority` tag
    pub fn priority(mut self, val: f32) -> UrlEntryBuilder {
        self.url_entry.priority = Priority::Value(val);
        return self;
    }

    /// Builds `UrlEntry` structure
    pub fn build(self) -> Result<UrlEntry, Error> {
        // TODO: add check for at least the name.
        if !self.url_entry.loc.is_url() {
            return Err(Error::Invalid("Required a location in the Url".to_string()));
        }
        if let Priority::Value(val) = self.url_entry.priority {
            if val > 1.0 || val < 0.0 {
                return Err(Error::Invalid("priority should be betwheen 0 and 1".to_string()))
            }
        }
        return Ok(self.url_entry);
    }
}

impl Into<UrlEntry> for UrlEntryBuilder {
    /// Panics when builder is misconfigured.
    fn into(self) -> UrlEntry {
        return self.build().unwrap();
    }
}

impl Into<UrlEntry> for Url {
    /// Notably does not panic
    fn into( self ) -> UrlEntry {
        UrlEntry {
            loc: Location::from(self),
            lastmod: LastMod::None,
            changefreq: ChangeFreq::None,
            priority: Priority::None,
        }
    }
}

impl Into<UrlEntry> for String {
    /// Panics when url is invalid
    fn into(self) -> UrlEntry {
        let location = Location::from(self);
        if let Location::ParseErr(error) = location {
            panic!("Unable to parse location: {}", error);
        }
        UrlEntry {
            loc: location,
            lastmod: LastMod::None,
            changefreq: ChangeFreq::None,
            priority: Priority::None,
        }
    }
}

impl Into<UrlEntry> for &'static str {
    /// Panics when url is invalid
    fn into(self) -> UrlEntry {
        let location: String = self.into();
        return location.into();
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

    /// Creates builder for `SiteMapEntry` structure
    pub fn builder() -> SiteMapEntryBuilder {
        SiteMapEntryBuilder { sitemap_entry: SiteMapEntry::new() }
    }
}


/// Builds `SiteMapEntry` structure
#[derive(Debug,Clone)]
pub struct SiteMapEntryBuilder {
    sitemap_entry: SiteMapEntry,
}

impl SiteMapEntryBuilder {
    /// Defines `loc` tag
    pub fn loc<S: Into<String>>(mut self, url: S) -> SiteMapEntryBuilder {
        let url = url.into();
        self.sitemap_entry.loc = Location::from(url);
        return self;
    }

    /// Defines `lastmod` tag
    pub fn lastmod(mut self, date: DateTime<FixedOffset>) -> SiteMapEntryBuilder {
        self.sitemap_entry.lastmod = LastMod::DateTime(date);
        return self;
    }

    /// Builds `SiteMapEntry` structure
    pub fn build(self) -> Result<SiteMapEntry, Error> {
        // TODO: add check for at least the name.
        if let Location::Url(_) = self.sitemap_entry.loc {
            Ok(self.sitemap_entry)
        } else {
            Err(Error::Invalid("Required a location in the sitemap".to_string()))
        }
    }
}

impl Into<SiteMapEntry> for SiteMapEntryBuilder {
    /// Panics when builder is misconfigured.
    fn into(self) -> SiteMapEntry {
        return self.build().unwrap();
    }
}

impl Into<SiteMapEntry> for Url {
    /// Notably does not panic
    fn into( self ) -> SiteMapEntry {
        SiteMapEntry {
            loc: Location::from( self ),
            lastmod: LastMod::None,
        }
    }
}

impl Into<SiteMapEntry> for String {
    /// Panics when url is invalid
    fn into(self) -> SiteMapEntry {
        let location = Location::from(self);
        if let Location::ParseErr(error) = location {
            panic!("Unable to parse location: {}", error);
        }
        SiteMapEntry {
            loc: location,
            lastmod: LastMod::None,
        }
    }
}

impl Into<SiteMapEntry> for &'static str {
    /// Panics when url is invalid
    fn into(self) -> SiteMapEntry {
        let location: String = self.into();
        return location.into();
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
    ParseErr(url::ParseError),
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

    /// Checks is location equals url
    pub fn is_url(&self) -> bool {
        return match *self {
            Location::Url(_) => true,
            _ => false,
        }
    }

    /// Checks is location equals none
    pub fn is_none(&self) -> bool {
        return match *self {
            Location::None => true,
            _ => false,
        }
    }

    /// Checks is location contains parse error.
    pub fn is_parse_error(&self) -> bool {
        return match *self {
            Location::ParseErr(_) => true,
            _ => false,
        }
    }
}
impl From<Url> for Location {
    ///Wraps a Url into a Location enum
    fn from( url: Url ) -> Self {
        Location::Url( url )
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
                return Location::ParseErr(error);
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
    ParseErr(chrono_utils::parser::error::ParseError),
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
                return LastMod::ParseErr(error);
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
    ParseErr(ChangeFreqParseError),
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
            ChangeFreq::ParseErr(_) => "",
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
                return ChangeFreq::ParseErr(ChangeFreqParseError::new(time));
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
    ParseErr(num::ParseFloatError),
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
                return Priority::ParseErr(error);
            }
        }
    }
}
