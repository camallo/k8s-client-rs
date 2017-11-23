/// Basic Kubernetes client, not version-specific.

use futures::{Future, Stream};
use hyper::{self, client};
use hyper_rustls;
use super::StreamBlob;
use super::errors::*;

use std::str::FromStr;

/// Basic Kubernetes client, non-versioned.
#[derive(Debug, Clone)]
pub struct ClientBasic {
    pub(crate) base_url: String,
    pub(crate) hclient: hyper::Client<hyper_rustls::HttpsConnector>,
    pub(crate) hostname: String,
}

impl ClientBasic {
    pub(crate) fn new_request(&self, method: hyper::Method, url: hyper::Uri) -> hyper::client::Request {
        let mut req = client::Request::new(method, url);
        let host = hyper::header::Host::new(self.hostname.clone(), None);
        req.headers_mut().set(host);
        req
    }

    fn basic_api(&self, ep: &str) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}{}", self.base_url, ep);
            try!(hyper::Uri::from_str(s.as_str()))
        };
        let req = self.new_request(hyper::Method::Get, url.clone());
        let freq = self.hclient.request(req);
        let fres = freq.from_err()
            .map(move |r| {
                trace!("GET {:?}", url);
                r
            })
            .and_then(|r| {
                if r.status() != hyper::StatusCode::Ok {
                    return Err(hyper::Error::Status).chain_err(|| format!("{}", r.status()));
                };
                Ok(r.body().from_err())
            })
            .into_stream()
            .flatten();
        Ok(Box::new(fres))
    }

    /// Get cluster API version.
    pub fn version(&self) -> Result<StreamBlob> {
        self.basic_api("/version")
    }

    /// Get cluster health.
    pub fn health(&self) -> Result<StreamBlob> {
        self.basic_api("/healthz")
    }

    /// Get cluster API root paths.
    pub fn root_paths(&self) -> Result<StreamBlob> {
        self.basic_api("/")
    }
}
