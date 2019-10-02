use sitemap::reader::{SiteMapReader, SiteMapEntity};
use url::Url;
use std::fs::File;
use chrono::DateTime;
use sitemap::structs::{ChangeFreq, SiteMapEntry, UrlEntry};
use xml::reader;
fn read_sitemap(file_name: &str) -> (Vec<UrlEntry>, Vec<SiteMapEntry>, Vec<reader::Error>) {
    let mut urls = Vec::new();
    let mut sitemaps = Vec::new();
    let mut errors = Vec::new();
    let file = File::open(file_name).unwrap();
    let parser = SiteMapReader::new(file);
    for entity in parser {
        match entity {
            SiteMapEntity::Url(url_entry) => {
                urls.push(url_entry);
            }
            SiteMapEntity::SiteMap(sitemap_entry) => {
                sitemaps.push(sitemap_entry);
            }
            SiteMapEntity::Err(error) => {
                errors.push(error);
            }
        }
    }
    return (urls, sitemaps, errors);
}
#[test]
fn test_read_sitemap_check_urls() {
    let (urls, _, errors) = read_sitemap("tests/documents/sitemap1.xml");
    assert_eq!(urls.len(), 5);
    assert_eq!(errors.len(), 0);
    // no 0
    assert_eq!(urls[0].loc.get_url().unwrap(),
               Url::parse("http://www.example.com/").unwrap());
    assert_eq!(urls[0].lastmod.get_time().unwrap(),
               DateTime::parse_from_rfc3339("2005-01-01T00:00:00+00:00").unwrap());
    assert_eq!(urls[0].changefreq, ChangeFreq::Monthly);
    assert_eq!(urls[0].priority.get_priority().unwrap(), 0.8);

    // no 1
    assert_eq!(urls[1].loc.get_url().unwrap(),
               Url::parse("http://www.example.com/catalog?item=12&desc=vacation_hawaii").unwrap());
    assert_eq!(urls[1].changefreq, ChangeFreq::Weekly);

    // no 2
    assert_eq!(urls[2].loc.get_url().unwrap(),
               Url::parse("http://www.example.com/catalog?item=73&desc=vacation_new_zealand")
                   .unwrap());
    assert_eq!(urls[2].lastmod.get_time().unwrap(),
               DateTime::parse_from_rfc3339("2004-12-23T00:00:00+00:00").unwrap());
    assert_eq!(urls[2].changefreq, ChangeFreq::Weekly);

    // no 3
    assert_eq!(urls[3].loc.get_url().unwrap(),
               Url::parse("http://www.example.com/catalog?item=74&desc=vacation_newfoundland")
                   .unwrap());
    assert_eq!(urls[3].lastmod.get_time().unwrap(),
               DateTime::parse_from_rfc3339("2004-12-23T18:00:15+00:00").unwrap());
    assert_eq!(urls[3].priority.get_priority().unwrap(), 0.3);

    // no 4
    assert_eq!(urls[4].loc.get_url().unwrap(),
               Url::parse("http://www.example.com/catalog?item=83&desc=vacation_usa").unwrap());
    assert_eq!(urls[4].lastmod.get_time().unwrap(),
               DateTime::parse_from_rfc3339("2004-11-23T00:00:00+00:00").unwrap());
}

#[test]
fn test_read_sitemap_check_sitemaps() {
    let (_, sitemaps, errors) = read_sitemap("tests/documents/sitemap1.xml");
    assert_eq!(sitemaps.len(), 2);
    assert_eq!(errors.len(), 0);
    // Sitemap no 0

    assert_eq!(sitemaps[0].loc.get_url().unwrap(),
               Url::parse("http://www.example.com/sitemap1.xml.gz").unwrap());
    assert_eq!(sitemaps[0].lastmod.get_time().unwrap(),
               DateTime::parse_from_rfc3339("2004-10-01T18:23:17+00:00").unwrap());
    // Sitemap no 1

    assert_eq!(sitemaps[1].loc.get_url().unwrap(),
               Url::parse("http://www.example.com/sitemap2.xml.gz").unwrap());
    assert_eq!(sitemaps[1].lastmod.get_time().unwrap(),
               DateTime::parse_from_rfc3339("2005-01-01T00:00:00+00:00").unwrap());
}
