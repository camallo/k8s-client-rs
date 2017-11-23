use futures::{Future, Stream};
use hyper;

use ::StreamBlob;
use ::errors::*;

use std::str::FromStr;

impl super::Client {

    /// get available resources

    pub fn get_apps_v1beta1_apiresources(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/");
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

    /// list or watch objects of kind Deployment

    pub fn list_apps_v1beta1_deployment_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/deployments");
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

    /// list or watch objects of kind Deployment

    pub fn list_apps_v1beta1_namespaced_deployment(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/namespaces/{namespace}/deployments");
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

    /// list or watch objects of kind StatefulSet

    pub fn list_apps_v1beta1_namespaced_stateful_set(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/namespaces/{namespace}/statefulsets");
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

    /// list or watch objects of kind StatefulSet

    pub fn list_apps_v1beta1_stateful_set_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/statefulsets");
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

    /// read the specified Deployment

    pub fn read_apps_v1beta1_namespaced_deployment(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}");
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

    /// read status of the specified Deployment

    pub fn read_apps_v1beta1_namespaced_deployment_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/status");
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

    /// read scale of the specified Scale

    pub fn read_apps_v1beta1_namespaced_scale_scale(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale");
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

    /// read the specified StatefulSet

    pub fn read_apps_v1beta1_namespaced_stateful_set(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}");
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

    /// read status of the specified StatefulSet

    pub fn read_apps_v1beta1_namespaced_stateful_set_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/status");
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

    /// watch individual changes to a list of Deployment

    pub fn watch_apps_v1beta1_deployment_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/watch/deployments");
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

    /// watch changes to an object of kind Deployment

    pub fn watch_apps_v1beta1_namespaced_deployment(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/watch/namespaces/{namespace}/deployments/{name}");
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

    /// watch individual changes to a list of Deployment

    pub fn watch_apps_v1beta1_namespaced_deployment_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/watch/namespaces/{namespace}/deployments");
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

    /// watch changes to an object of kind StatefulSet

    pub fn watch_apps_v1beta1_namespaced_stateful_set(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/watch/namespaces/{namespace}/statefulsets/{name}");
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

    /// watch individual changes to a list of StatefulSet

    pub fn watch_apps_v1beta1_namespaced_stateful_set_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/watch/namespaces/{namespace}/statefulsets");
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

    /// watch individual changes to a list of StatefulSet

    pub fn watch_apps_v1beta1_stateful_set_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/apps/v1beta1/watch/statefulsets");
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
