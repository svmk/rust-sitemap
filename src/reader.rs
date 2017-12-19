//! Contains sitemap reader.
//!
//! # Examples
//!
//! Reading sitemap from file.
//!
//! ```rust
//! extern crate sitemap;
//! use sitemap::reader::{SiteMapReader,SiteMapEntity};
//! use std::fs::File;
//! fn main() {
//!    let mut urls = Vec::new();
//! 	let mut sitemaps = Vec::new();
//! 	let mut errors = Vec::new();
//! 	let file = File::open("tests/documents/sitemap1.xml").expect("Unable to open file.");
//! 	let parser = SiteMapReader::new(file);
//! 	for entity in parser {
//! 		match entity {
//! 			SiteMapEntity::Url(url_entry) => {
//! 				urls.push(url_entry);
//! 			},
//! 			SiteMapEntity::SiteMap(sitemap_entry) => {
//! 				sitemaps.push(sitemap_entry);
//! 			},
//! 			SiteMapEntity::Err(error) => {
//! 				errors.push(error);
//! 			},
//! 		}
//! 	}
//! 	println!("urls = {:?}",urls);
//! 	println!("sitemaps = {:?}",sitemaps);
//! 	println!("errors = {:?}",errors);
//! }
//! ```
use structs;
use xml;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent, Events};
use std::convert::From;

/// A wrapper around an `std::io::Read` instance which provides sitemap parsing.
///
/// It's provides iterator over `SiteMapEntity`.
pub struct SiteMapReader<T: Read + Sized> {
    path: Vec<String>,
    url_item: structs::UrlEntry,
    sitemap_item: structs::SiteMapEntry,
    parser: Events<T>,
}
/// Sitemap entry.
#[derive(Debug,Clone)]
pub enum SiteMapEntity {
    /// Url entry.
    Url(structs::UrlEntry),
    /// Sitemap entry.
    SiteMap(structs::SiteMapEntry),
    /// XML read error.
    Err(xml::reader::Error),
}
impl<T: Read + Sized> SiteMapReader<T> {
    /// Creates a new reader, consuming the given stream.
    pub fn new(source: T) -> SiteMapReader<T> {
        let parser = EventReader::new(source).into_iter();
        SiteMapReader {
            path: Vec::new(),
            url_item: structs::UrlEntry::new(),
            sitemap_item: structs::SiteMapEntry::new(),
            parser: parser,
        }
    }
    fn open_tag(&mut self) {
        if self.path == vec!["urlset", "url"] {
            self.url_item = structs::UrlEntry::new();
        } else if self.path == vec!["sitemapindex", "sitemap"] {
            self.sitemap_item = structs::SiteMapEntry::new();
        }
    }
    fn text_content(&mut self, data: String) {
        if self.path == vec!["urlset", "url", "loc"] {
            self.url_item.loc = structs::Location::from(data);
        } else if self.path == vec!["urlset", "url", "lastmod"] {
            self.url_item.lastmod = structs::LastMod::from(data);
        } else if self.path == vec!["urlset", "url", "changefreq"] {
            self.url_item.changefreq = structs::ChangeFreq::from(data);
        } else if self.path == vec!["urlset", "url", "priority"] {
            self.url_item.priority = structs::Priority::from(data);
        } else if self.path == vec!["sitemapindex", "sitemap", "loc"] {
            self.sitemap_item.loc = structs::Location::from(data);
        } else if self.path == vec!["sitemapindex", "sitemap", "lastmod"] {
            self.sitemap_item.lastmod = structs::LastMod::from(data);
        }
    }
    fn close_tag(&mut self) -> Option<SiteMapEntity> {
        if self.path == vec!["urlset", "url"] {
            return Some(SiteMapEntity::Url(self.url_item.clone()));
        } else if self.path == vec!["sitemapindex", "sitemap"] {
            return Some(SiteMapEntity::SiteMap(self.sitemap_item.clone()));
        }
        return None;
    }
}
impl<T: Read + Sized> Iterator for SiteMapReader<T> {
    type Item = SiteMapEntity;
    fn next(&mut self) -> Option<SiteMapEntity> {
        loop {
            let e = self.parser.next();
            match e {
                Some(e) => {
                    match e {
                        Ok(XmlEvent::StartElement { name, .. }) => {
                            let tag_name = name.local_name.to_lowercase();
                            self.path.push(tag_name.to_string());
                            self.open_tag();
                        }
                        Ok(XmlEvent::EndElement { .. }) => {
                            let entity = self.close_tag();
                            self.path.pop();
                            match entity {
                                Some(entity) => {
                                    return Some(entity);
                                }
                                None => {}
                            }
                        }
                        Ok(XmlEvent::Characters(data)) => {
                            self.text_content(data);
                        }
                        Err(error) => {
                            return Some(SiteMapEntity::Err(error));
                        }
                        _ => {}
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}
