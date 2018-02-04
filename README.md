# sitemap, an Sitemap library for Rust

[![Chrono-utils on Travis CI][travis-image]][travis]
[![Chrono-utils on crates.io][cratesio-image]][cratesio]

[travis-image]: https://travis-ci.org/svmk/rust-sitemap.svg?branch=master
[travis]: https://travis-ci.org/svmk/rust-sitemap
[cratesio-image]: https://img.shields.io/crates/v/sitemap.svg
[cratesio]: https://crates.io/crates/sitemap


sitemap is an library for [Rust](http://www.rust-lang.org/) programming language.

## features
* Streaming reading sitemap

## Restrictions
* no other encodings but UTF-8 are supported yet
* validation is not supported

## Building and using
sitemap uses Cargo, so just add a dependency section in your project's manifest:
```
[dependencies]
sitemap = "0.1.1"
```
The package exposes a single crate called sitemap:
```rust
extern crate sitemap;
```

## Reading sitemap documents
`sitemap::reader::SiteMapReader` requires a `Read` instance to read from. When a proper stream-based encoding library is available, it is likely that sitemap will be switched to use whatever character stream structure this library would provide, but currently it is a `Read`.

Using `SiteMapReader` is very straightforward. Just provide a `Read` instance to obtain an iterator over events:

```rust
extern crate sitemap;
use sitemap::reader::{SiteMapReader,SiteMapEntity};
use std::fs::File;
fn main() {
    let mut urls = Vec::new();
    let mut sitemaps = Vec::new();
    let mut errors = Vec::new();
    let file = File::open("sitemap.xml").expect("Unable to open file.");
    let parser = SiteMapReader::new(file);
    for entity in parser {
        match entity {
            SiteMapEntity::Url(url_entry) => {
                urls.push(url_entry);
            },
            SiteMapEntity::SiteMap(sitemap_entry) => {
                sitemaps.push(sitemap_entry);
            },
            SiteMapEntity::Err(error) => {
                errors.push(error);
            },
        }
    }
    println!("urls = {:?}",urls);
    println!("sitemaps = {:?}",sitemaps);
    println!("errors = {:?}",errors);
}
```

## Writng sitemap documents
```rust
extern crate sitemap;
use sitemap::writer::SiteMapWriter;
use sitemap::structs::UrlEntry;
use std::io::stdout;
fn main() {
    let mut output = stdout();
    let sitemap_writer = SiteMapWriter::new(&mut output);
    let mut urlwriter = sitemap_writer.start_urlset().expect("Unable to write urlset");
    urlwriter.url("http://github.com").expect("Unable to write url");
    urlwriter.url(UrlEntry::builder().loc("http://google.com")).expect("Unable to write url");
    urlwriter.url(UrlEntry::builder().loc("http://yandex.ru").build().unwrap()).expect("Unable to write url");
    urlwriter.end().expect("Unable to write close tags");
}
```

## Roadmap
Highest priority first, approximately.
1. Sitemap writer - done
2. Sitemap validation

## Known issues
All known issues are present on GitHub issue tracker: http://github.com/svmk/sitemap/issues. Feel free to post any found problems there.

## License

This library is licensed under MIT license.


