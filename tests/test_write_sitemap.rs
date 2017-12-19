extern crate sitemap;
extern crate chrono;

use sitemap::writer::SiteMapWriter;
use sitemap::structs::{UrlEntry, ChangeFreq, SiteMapEntry};
use chrono::{DateTime, NaiveDate, FixedOffset};

static CONTENT: &str =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<urlset>
  <url>
    \
     <loc>http://www.example.com/index.html</loc>
    \
     <lastmod>2016-07-08T09:10:11+00:00</lastmod>
    <changefreq>daily</changefreq>
    \
     <lastmod>0.2</lastmod>
  </url>
  <url>
    <loc>http://www.example.com/other.html</loc>
    \
     <lastmod>2016-07-18T09:10:11+00:00</lastmod>
    <changefreq>monthly</changefreq>
    \
     <lastmod>0.1</lastmod>
  </url>
</urlset>
<sitemapindex>
  <sitemap>
    \
     <loc>http://www.example.com/other_sitemap.xml</loc>
    \
     <lastmod>2016-07-18T09:10:11+00:00</lastmod>
  </sitemap>
</sitemapindex>";


#[test]
fn test_write_sitemap() {
    let mut output = Vec::<u8>::new();
    {
        let sitemap_writer = SiteMapWriter::new(&mut output);
        let mut urlwriter = sitemap_writer.start_urlset().expect("can write the file");
        let date = DateTime::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
                                      FixedOffset::east(0));
        let url_entry = UrlEntry::builder()
            .loc("http://www.example.com/index.html".into())
            .changefreq(ChangeFreq::Daily)
            .priority(0.2)
            .lastmod(date)
            .build();
        urlwriter.url(url_entry).expect("can write the file");
        let date1 = DateTime::from_utc(NaiveDate::from_ymd(2016, 7, 18).and_hms(9, 10, 11),
                                       FixedOffset::east(0));
        let url_entry = UrlEntry::builder()
            .loc("http://www.example.com/other.html".into())
            .changefreq(ChangeFreq::Monthly)
            .priority(0.1)
            .lastmod(date1)
            .build();
        urlwriter.url(url_entry).expect("can write the file");
        let sitemap_writer = urlwriter.end().expect("close the urlset block");

        let mut sitemap_index_writer = sitemap_writer.start_sitemapindex()
            .expect("start sitemap index tag");
        let sitemap_entry = SiteMapEntry::builder()
            .loc("http://www.example.com/other_sitemap.xml".into())
            .lastmod(date1)
            .build();
        sitemap_index_writer.sitemap(sitemap_entry).expect("can write the file");
        sitemap_index_writer.end().expect("close sitemap block");
    }
    assert_eq!(output, CONTENT.as_bytes());
}
