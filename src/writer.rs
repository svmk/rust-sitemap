//! Contains sitemap writer.
//!
//! # Examples
//!
//! Writing sitemap into stdout.
//!
//! ```rust
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
use xml::writer::{EventWriter, EmitterConfig, XmlEvent};
use crate::Error;
use crate::structs::{UrlEntry, Location, LastMod, ChangeFreq, Priority, SiteMapEntry};

const DEFAULT_NAMESPACE: &str = "http://www.sitemaps.org/schemas/sitemap/0.9";

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

    /// Starts writing urls with sitemap namespace
    /// Adds namespace attribute `http://www.sitemaps.org/schemas/sitemap/0.9` for `urlset` tag
    pub fn start_urlset(self) -> Result<UrlSetWriter<T>, Error> {
        return self.start_urlset_ns(DEFAULT_NAMESPACE);
    }

    /// Starts writing urls with custom sitemap namespace
    /// Adds specified namespace attribute for `urlset` tag
    pub fn start_urlset_ns(mut self, namespace: &str) -> Result<UrlSetWriter<T>, Error> {
        self.writer.write(XmlEvent::start_element("urlset").default_ns(namespace))?;
        Ok(UrlSetWriter { sitemap: self })
    }

    /// Starts writing urls without namespace
    pub fn start_urlset_without_ns(mut self) -> Result<UrlSetWriter<T>, Error> {
        self.writer.write(XmlEvent::start_element("urlset"))?;
        Ok(UrlSetWriter { sitemap: self })
    }

    /// Starts writing sitemap urls
    /// Adds namespace attribute `http://www.sitemaps.org/schemas/sitemap/0.9` for `sitemapindex` tag
    pub fn start_sitemapindex(self) -> Result<SiteMapIndexWriter<T>, Error> {
        return self.start_sitemapindex_ns(DEFAULT_NAMESPACE);
    }

    /// Starts writing sitemap urls with custom sitemap namespace
    /// Adds specified namespace attribute for `sitemapindex` tag
    pub fn start_sitemapindex_ns(
        mut self,
        namespace: &str,
    ) -> Result<SiteMapIndexWriter<T>, Error> {
        self.writer.write(XmlEvent::start_element("sitemapindex").default_ns(namespace))?;
        Ok(SiteMapIndexWriter { sitemap: self })
    }

    /// Starts writing sitemap urls without namespace
    pub fn start_sitemapindex_without_ns(mut self) -> Result<SiteMapIndexWriter<T>, Error> {
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
