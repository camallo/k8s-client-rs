use futures::{Future, Stream};
use hyper;

use ::StreamBlob;
use ::errors::*;

use std::str::FromStr;

impl super::Client {

    /// get available resources

    pub fn get_storage_v1_apiresources(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/storage.k8s.io/v1/");
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

    /// list or watch objects of kind StorageClass

    pub fn list_storage_v1_storage_class(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/storage.k8s.io/v1/storageclasses");
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

    /// read the specified StorageClass

    pub fn read_storage_v1_storage_class(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/storage.k8s.io/v1/storageclasses/{name}");
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

    /// watch changes to an object of kind StorageClass

    pub fn watch_storage_v1_storage_class(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/storage.k8s.io/v1/watch/storageclasses/{name}");
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

    /// watch individual changes to a list of StorageClass

    pub fn watch_storage_v1_storage_class_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/storage.k8s.io/v1/watch/storageclasses");
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
