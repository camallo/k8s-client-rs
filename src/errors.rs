//! Error types and traits for this library.
use hyper;
use rustls;
use serde_yaml;
use serde_json;
use std::io;

error_chain!{
    foreign_links {
        Http(hyper::Error);
        Io(io::Error);
        Tls(rustls::TLSError);
        Uri(hyper::error::UriError);
        Yaml(serde_yaml::Error);
        Json(serde_json::Error);
    }
}
