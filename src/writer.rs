
use std::io::Write;
use Error;
use xml::writer::{EventWriter, EmitterConfig, XmlEvent};
use structs::{UrlEntry, Location, LastMod, ChangeFreq, Priority, SiteMapEntry};

pub struct SiteMapWriter<T: Write + Sized> {
    writer: EventWriter<T>,
}

pub struct UrlSetWriter<T: Write + Sized> {
    sitemap: SiteMapWriter<T>,
}

impl<T: Write + Sized> UrlSetWriter<T> {
    pub fn url(&mut self, url: UrlEntry) -> Result<(), Error> {
        self.sitemap.writer.write(XmlEvent::start_element("url"))?;
        if let Location::Url(loc) = url.loc {
            self.sitemap.write_content_element("loc", loc.as_str())?;
        }
        if let LastMod::DateTime(lastmod) = url.lastmod {
            self.sitemap.write_content_element("lastmod", lastmod.to_rfc3339().as_str())?;
        }
        match url.changefreq {
            ChangeFreq::Err(_) => {}
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

    pub fn end(mut self) -> Result<SiteMapWriter<T>, Error> {
        self.sitemap.writer.write(XmlEvent::end_element().name("urlset"))?;
        Ok(self.sitemap)
    }
}

pub struct SiteMapIndexWriter<T: Write + Sized> {
    sitemap: SiteMapWriter<T>,
}

impl<T: Write + Sized> SiteMapIndexWriter<T> {
    pub fn sitemap(&mut self, sitemapentry: SiteMapEntry) -> Result<(), Error> {
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

    pub fn end(mut self) -> Result<SiteMapWriter<T>, Error> {
        self.sitemap.writer.write(XmlEvent::end_element().name("sitemapindex"))?;
        Ok(self.sitemap)
    }
}

impl<T: Write + Sized> SiteMapWriter<T> {
    pub fn new(out: T) -> SiteMapWriter<T> {
        let writer = EmitterConfig::new().perform_indent(true).create_writer(out);
        SiteMapWriter { writer: writer }
    }

    pub fn start_urlset(mut self) -> Result<UrlSetWriter<T>, Error> {
        self.writer.write(XmlEvent::start_element("urlset"))?;
        Ok(UrlSetWriter { sitemap: self })
    }

    pub fn write_content_element(&mut self, ele: &str, content: &str) -> Result<(), Error> {
        self.writer.write(XmlEvent::start_element(ele))?;
        self.writer.write(XmlEvent::characters(content))?;
        self.writer.write(XmlEvent::end_element().name(ele))?;
        Ok(())
    }
    pub fn start_sitemapindex(mut self) -> Result<SiteMapIndexWriter<T>, Error> {
        self.writer.write(XmlEvent::start_element("sitemapindex"))?;
        Ok(SiteMapIndexWriter { sitemap: self })
    }
}
