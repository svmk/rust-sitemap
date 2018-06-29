//! Contains sitemap writer.
//!
//! # Examples
//!
//! Writing sitemap into stdout.
//!
//! ```rust
//! extern crate sitemap;
//! use sitemap::writer::SiteMapWriter;
//! use sitemap::structs::UrlEntry;
//! use std::io::stdout;
//! fn main() {
//!     let mut output = stdout();
//!     let sitemap_writer = SiteMapWriter::new(&mut output);
//!     let mut urlwriter = sitemap_writer.start_urlset().expect("Unable to write urlset");
//!     urlwriter.url("http://github.com").expect("Unable to write url");
//!     urlwriter.url(UrlEntry::builder().loc("http://google.com")).expect("Unable to write url");
//!     urlwriter.url(UrlEntry::builder().loc("http://yandex.ru").build().unwrap()).expect("Unable to write url");
//!     urlwriter.end().expect("Unable to write close tags");
//! }
//! ```
use std::io::Write;
use Error;
use xml::writer::{EventWriter, EmitterConfig, XmlEvent};
use structs::{UrlEntry, Location, LastMod, ChangeFreq, Priority, SiteMapEntry};

/// Writes xml tags into writer.
pub struct SiteMapWriter<T: Write + Sized> {
    writer: EventWriter<T>,
}

impl<T: Write + Sized> SiteMapWriter<T> {
    /// Creates new sitemap writer
    pub fn new(out: T) -> SiteMapWriter<T> {
        let writer = EmitterConfig::new().perform_indent(true).create_writer(out);
        SiteMapWriter { writer: writer }
    }

    /// Starts writing urls
    pub fn start_urlset(mut self) -> Result<UrlSetWriter<T>, Error> {
        self.writer.write(XmlEvent::start_element("urlset").default_ns("http://www.sitemaps.org/schemas/sitemap/0.9"))?;
        Ok(UrlSetWriter { sitemap: self })
    }

    /// Starts writing sitemap urls.
    pub fn start_sitemapindex(mut self) -> Result<SiteMapIndexWriter<T>, Error> {
        self.writer.write(XmlEvent::start_element("sitemapindex"))?;
        Ok(SiteMapIndexWriter { sitemap: self })
    }

    fn write_content_element(&mut self, ele: &str, content: &str) -> Result<(), Error> {
        self.writer.write(XmlEvent::start_element(ele))?;
        self.writer.write(XmlEvent::characters(content))?;
        self.writer.write(XmlEvent::end_element().name(ele))?;
        Ok(())
    }
}

/// Writes urls into writer.
pub struct UrlSetWriter<T: Write + Sized> {
    sitemap: SiteMapWriter<T>,
}

impl<T: Write + Sized> UrlSetWriter<T> {
    /// Writes page url.
    pub fn url<S: Into<UrlEntry>>(&mut self, url: S) -> Result<(), Error> {
        let url = url.into();
        self.sitemap.writer.write(XmlEvent::start_element("url"))?;
        if let Location::Url(loc) = url.loc {
            self.sitemap.write_content_element("loc", loc.as_str())?;
        }
        if let LastMod::DateTime(lastmod) = url.lastmod {
            self.sitemap.write_content_element("lastmod", lastmod.to_rfc3339().as_str())?;
        }
        match url.changefreq {
            ChangeFreq::ParseErr(_) => {}
            ChangeFreq::None => {}
            _ => {
                self.sitemap.write_content_element("changefreq", url.changefreq.as_str())?;
            }
        }
        if let Priority::Value(priority) = url.priority {
            self.sitemap.write_content_element("lastmod", priority.to_string().as_str())?;
        }
        self.sitemap.writer.write(XmlEvent::end_element().name("url"))?;
        Ok(())
    }

    /// Completes writing data.
    pub fn end(mut self) -> Result<SiteMapWriter<T>, Error> {
        self.sitemap.writer.write(XmlEvent::end_element().name("urlset"))?;
        Ok(self.sitemap)
    }
}

/// Writes sitemaps list into writer.
pub struct SiteMapIndexWriter<T: Write + Sized> {
    sitemap: SiteMapWriter<T>,
}

impl<T: Write + Sized> SiteMapIndexWriter<T> {

    /// Writes sitemap entry.
    pub fn sitemap<S: Into<SiteMapEntry>>(&mut self, sitemapentry: S) -> Result<(), Error> {
        let sitemapentry = sitemapentry.into();
        self.sitemap.writer.write(XmlEvent::start_element("sitemap"))?;
        if let Location::Url(loc) = sitemapentry.loc {
            self.sitemap.write_content_element("loc", loc.as_str())?;
        }
        if let LastMod::DateTime(lastmod) = sitemapentry.lastmod {
            self.sitemap.write_content_element("lastmod", lastmod.to_rfc3339().as_str())?;
        }
        self.sitemap.writer.write(XmlEvent::end_element().name("sitemap"))?;
        Ok(())
    }

    /// Completes writing data.
    pub fn end(mut self) -> Result<SiteMapWriter<T>, Error> {
        self.sitemap.writer.write(XmlEvent::end_element().name("sitemapindex"))?;
        Ok(self.sitemap)
    }
}
