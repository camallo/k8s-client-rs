use futures::{Future, Stream};
use hyper;

use ::StreamBlob;
use ::errors::*;

use std::str::FromStr;

impl super::Client {

    /// get information of a group

    pub fn get_batch_apigroup(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/");
            try!(hyper::Uri::from_str(s.as_str()))
        };
        let req = self.new_request(hyper::Method::Get, url.clone());
        let freq = self.0.hclient.request(req);
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

}
