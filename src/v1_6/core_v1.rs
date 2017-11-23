use futures::{Future, Stream};
use hyper;

use ::StreamBlob;
use ::errors::*;

use std::str::FromStr;

impl super::Client {

    /// connect GET requests to attach of Pod

    pub fn connect_core_v1_get_namespaced_pod_attach(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/pods/{name}/attach");
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

    /// connect GET requests to exec of Pod

    pub fn connect_core_v1_get_namespaced_pod_exec(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/pods/{name}/exec");
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

    /// connect GET requests to portforward of Pod

    pub fn connect_core_v1_get_namespaced_pod_portforward(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/pods/{name}/portforward");
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

    /// connect GET requests to proxy of Pod

    pub fn connect_core_v1_get_namespaced_pod_proxy(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/pods/{name}/proxy");
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

    /// connect GET requests to proxy of Pod

    pub fn connect_core_v1_get_namespaced_pod_proxy_with_path(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}");
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

    /// connect GET requests to proxy of Service

    pub fn connect_core_v1_get_namespaced_service_proxy(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/services/{name}/proxy");
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

    /// connect GET requests to proxy of Service

    pub fn connect_core_v1_get_namespaced_service_proxy_with_path(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/services/{name}/proxy/{path}");
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

    /// connect GET requests to proxy of Node

    pub fn connect_core_v1_get_node_proxy(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/nodes/{name}/proxy");
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

    /// connect GET requests to proxy of Node

    pub fn connect_core_v1_get_node_proxy_with_path(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/nodes/{name}/proxy/{path}");
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

    /// get available resources

    pub fn get_core_v1_apiresources(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/");
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

    /// list objects of kind ComponentStatus

    pub fn list_core_v1_component_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/componentstatuses");
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

    /// list or watch objects of kind ConfigMap

    pub fn list_core_v1_config_map_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/configmaps");
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

    /// list or watch objects of kind Endpoints

    pub fn list_core_v1_endpoints_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/endpoints");
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

    /// list or watch objects of kind Event

    pub fn list_core_v1_event_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/events");
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

    /// list or watch objects of kind LimitRange

    pub fn list_core_v1_limit_range_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/limitranges");
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

    /// list or watch objects of kind Namespace

    pub fn list_core_v1_namespace(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces");
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

    /// list or watch objects of kind ConfigMap

    pub fn list_core_v1_namespaced_config_map(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/configmaps");
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

    /// list or watch objects of kind Endpoints

    pub fn list_core_v1_namespaced_endpoints(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/endpoints");
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

    /// list or watch objects of kind Event

    pub fn list_core_v1_namespaced_event(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/events");
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

    /// list or watch objects of kind LimitRange

    pub fn list_core_v1_namespaced_limit_range(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/limitranges");
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

    /// list or watch objects of kind PersistentVolumeClaim

    pub fn list_core_v1_namespaced_persistent_volume_claim(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/persistentvolumeclaims");
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

    /// list or watch objects of kind Pod

    pub fn list_core_v1_namespaced_pod(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/pods");
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

    /// list or watch objects of kind PodTemplate

    pub fn list_core_v1_namespaced_pod_template(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/podtemplates");
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

    /// list or watch objects of kind ReplicationController

    pub fn list_core_v1_namespaced_replication_controller(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/replicationcontrollers");
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

    /// list or watch objects of kind ResourceQuota

    pub fn list_core_v1_namespaced_resource_quota(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/resourcequotas");
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

    /// list or watch objects of kind Secret

    pub fn list_core_v1_namespaced_secret(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/secrets");
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

    /// list or watch objects of kind Service

    pub fn list_core_v1_namespaced_service(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/services");
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

    /// list or watch objects of kind ServiceAccount

    pub fn list_core_v1_namespaced_service_account(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/serviceaccounts");
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

    /// list or watch objects of kind Node

    pub fn list_core_v1_node(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/nodes");
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

    /// list or watch objects of kind PersistentVolume

    pub fn list_core_v1_persistent_volume(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/persistentvolumes");
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

    /// list or watch objects of kind PersistentVolumeClaim

    pub fn list_core_v1_persistent_volume_claim_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/persistentvolumeclaims");
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

    /// list or watch objects of kind Pod

    pub fn list_core_v1_pod_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/pods");
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

    /// list or watch objects of kind PodTemplate

    pub fn list_core_v1_pod_template_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/podtemplates");
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

    /// list or watch objects of kind ReplicationController

    pub fn list_core_v1_replication_controller_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/replicationcontrollers");
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

    /// list or watch objects of kind ResourceQuota

    pub fn list_core_v1_resource_quota_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/resourcequotas");
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

    /// list or watch objects of kind Secret

    pub fn list_core_v1_secret_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/secrets");
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

    /// list or watch objects of kind ServiceAccount

    pub fn list_core_v1_service_account_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/serviceaccounts");
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

    /// list or watch objects of kind Service

    pub fn list_core_v1_service_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/services");
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

    /// proxy GET requests to Pod

    pub fn proxy_core_v1_getnamespaced_pod(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/proxy/namespaces/{namespace}/pods/{name}");
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

    /// proxy GET requests to Pod

    pub fn proxy_core_v1_getnamespaced_pod_with_path(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}");
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

    /// proxy GET requests to Service

    pub fn proxy_core_v1_getnamespaced_service(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/proxy/namespaces/{namespace}/services/{name}");
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

    /// proxy GET requests to Service

    pub fn proxy_core_v1_getnamespaced_service_with_path(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/proxy/namespaces/{namespace}/services/{name}/{path}");
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

    /// proxy GET requests to Node

    pub fn proxy_core_v1_getnode(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/proxy/nodes/{name}");
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

    /// proxy GET requests to Node

    pub fn proxy_core_v1_getnode_with_path(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/proxy/nodes/{name}/{path}");
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

    /// read the specified ComponentStatus

    pub fn read_core_v1_component_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/componentstatuses/{name}");
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

    /// read the specified Namespace

    pub fn read_core_v1_namespace(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{name}");
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

    /// read status of the specified Namespace

    pub fn read_core_v1_namespace_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{name}/status");
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

    /// read the specified ConfigMap

    pub fn read_core_v1_namespaced_config_map(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/configmaps/{name}");
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

    /// read the specified Endpoints

    pub fn read_core_v1_namespaced_endpoints(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/endpoints/{name}");
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

    /// read the specified Event

    pub fn read_core_v1_namespaced_event(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/events/{name}");
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

    /// read the specified LimitRange

    pub fn read_core_v1_namespaced_limit_range(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/limitranges/{name}");
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

    /// read the specified PersistentVolumeClaim

    pub fn read_core_v1_namespaced_persistent_volume_claim(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}");
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

    /// read status of the specified PersistentVolumeClaim

    pub fn read_core_v1_namespaced_persistent_volume_claim_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/persistentvolumeclaims/{name}/status");
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

    /// read the specified Pod

    pub fn read_core_v1_namespaced_pod(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/pods/{name}");
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

    /// read log of the specified Pod

    pub fn read_core_v1_namespaced_pod_log(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/pods/{name}/log");
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

    /// read status of the specified Pod

    pub fn read_core_v1_namespaced_pod_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/pods/{name}/status");
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

    /// read the specified PodTemplate

    pub fn read_core_v1_namespaced_pod_template(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/podtemplates/{name}");
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

    /// read the specified ReplicationController

    pub fn read_core_v1_namespaced_replication_controller(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/replicationcontrollers/{name}");
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

    /// read status of the specified ReplicationController

    pub fn read_core_v1_namespaced_replication_controller_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/replicationcontrollers/{name}/status");
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

    /// read the specified ResourceQuota

    pub fn read_core_v1_namespaced_resource_quota(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/resourcequotas/{name}");
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

    /// read status of the specified ResourceQuota

    pub fn read_core_v1_namespaced_resource_quota_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/resourcequotas/{name}/status");
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

    pub fn read_core_v1_namespaced_scale_scale(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/replicationcontrollers/{name}/scale");
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

    /// read the specified Secret

    pub fn read_core_v1_namespaced_secret(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/secrets/{name}");
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

    /// read the specified Service

    pub fn read_core_v1_namespaced_service(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/services/{name}");
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

    /// read the specified ServiceAccount

    pub fn read_core_v1_namespaced_service_account(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/serviceaccounts/{name}");
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

    /// read status of the specified Service

    pub fn read_core_v1_namespaced_service_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/namespaces/{namespace}/services/{name}/status");
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

    /// read the specified Node

    pub fn read_core_v1_node(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/nodes/{name}");
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

    /// read status of the specified Node

    pub fn read_core_v1_node_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/nodes/{name}/status");
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

    /// read the specified PersistentVolume

    pub fn read_core_v1_persistent_volume(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/persistentvolumes/{name}");
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

    /// read status of the specified PersistentVolume

    pub fn read_core_v1_persistent_volume_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/persistentvolumes/{name}/status");
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

    /// watch individual changes to a list of ConfigMap

    pub fn watch_core_v1_config_map_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/configmaps");
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

    /// watch individual changes to a list of Endpoints

    pub fn watch_core_v1_endpoints_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/endpoints");
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

    /// watch individual changes to a list of Event

    pub fn watch_core_v1_event_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/events");
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

    /// watch individual changes to a list of LimitRange

    pub fn watch_core_v1_limit_range_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/limitranges");
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

    /// watch changes to an object of kind Namespace

    pub fn watch_core_v1_namespace(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{name}");
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

    /// watch individual changes to a list of Namespace

    pub fn watch_core_v1_namespace_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces");
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

    /// watch changes to an object of kind ConfigMap

    pub fn watch_core_v1_namespaced_config_map(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/configmaps/{name}");
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

    /// watch individual changes to a list of ConfigMap

    pub fn watch_core_v1_namespaced_config_map_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/configmaps");
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

    /// watch changes to an object of kind Endpoints

    pub fn watch_core_v1_namespaced_endpoints(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/endpoints/{name}");
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

    /// watch individual changes to a list of Endpoints

    pub fn watch_core_v1_namespaced_endpoints_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/endpoints");
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

    /// watch changes to an object of kind Event

    pub fn watch_core_v1_namespaced_event(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/events/{name}");
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

    /// watch individual changes to a list of Event

    pub fn watch_core_v1_namespaced_event_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/events");
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

    /// watch changes to an object of kind LimitRange

    pub fn watch_core_v1_namespaced_limit_range(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/limitranges/{name}");
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

    /// watch individual changes to a list of LimitRange

    pub fn watch_core_v1_namespaced_limit_range_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/limitranges");
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

    /// watch changes to an object of kind PersistentVolumeClaim

    pub fn watch_core_v1_namespaced_persistent_volume_claim(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/persistentvolumeclaims/{name}");
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

    /// watch individual changes to a list of PersistentVolumeClaim

    pub fn watch_core_v1_namespaced_persistent_volume_claim_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/persistentvolumeclaims");
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

    /// watch changes to an object of kind Pod

    pub fn watch_core_v1_namespaced_pod(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/pods/{name}");
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

    /// watch individual changes to a list of Pod

    pub fn watch_core_v1_namespaced_pod_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/pods");
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

    /// watch changes to an object of kind PodTemplate

    pub fn watch_core_v1_namespaced_pod_template(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/podtemplates/{name}");
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

    /// watch individual changes to a list of PodTemplate

    pub fn watch_core_v1_namespaced_pod_template_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/podtemplates");
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

    /// watch changes to an object of kind ReplicationController

    pub fn watch_core_v1_namespaced_replication_controller(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/replicationcontrollers/{name}");
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

    /// watch individual changes to a list of ReplicationController

    pub fn watch_core_v1_namespaced_replication_controller_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/replicationcontrollers");
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

    /// watch changes to an object of kind ResourceQuota

    pub fn watch_core_v1_namespaced_resource_quota(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/resourcequotas/{name}");
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

    /// watch individual changes to a list of ResourceQuota

    pub fn watch_core_v1_namespaced_resource_quota_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/resourcequotas");
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

    /// watch changes to an object of kind Secret

    pub fn watch_core_v1_namespaced_secret(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/secrets/{name}");
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

    /// watch individual changes to a list of Secret

    pub fn watch_core_v1_namespaced_secret_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/secrets");
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

    /// watch changes to an object of kind Service

    pub fn watch_core_v1_namespaced_service(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/services/{name}");
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

    /// watch changes to an object of kind ServiceAccount

    pub fn watch_core_v1_namespaced_service_account(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/serviceaccounts/{name}");
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

    /// watch individual changes to a list of ServiceAccount

    pub fn watch_core_v1_namespaced_service_account_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/serviceaccounts");
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

    /// watch individual changes to a list of Service

    pub fn watch_core_v1_namespaced_service_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/namespaces/{namespace}/services");
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

    /// watch changes to an object of kind Node

    pub fn watch_core_v1_node(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/nodes/{name}");
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

    /// watch individual changes to a list of Node

    pub fn watch_core_v1_node_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/nodes");
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

    /// watch changes to an object of kind PersistentVolume

    pub fn watch_core_v1_persistent_volume(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/persistentvolumes/{name}");
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

    /// watch individual changes to a list of PersistentVolumeClaim

    pub fn watch_core_v1_persistent_volume_claim_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/persistentvolumeclaims");
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

    /// watch individual changes to a list of PersistentVolume

    pub fn watch_core_v1_persistent_volume_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/persistentvolumes");
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

    /// watch individual changes to a list of Pod

    pub fn watch_core_v1_pod_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/pods");
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

    /// watch individual changes to a list of PodTemplate

    pub fn watch_core_v1_pod_template_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/podtemplates");
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

    /// watch individual changes to a list of ReplicationController

    pub fn watch_core_v1_replication_controller_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/replicationcontrollers");
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

    /// watch individual changes to a list of ResourceQuota

    pub fn watch_core_v1_resource_quota_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/resourcequotas");
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

    /// watch individual changes to a list of Secret

    pub fn watch_core_v1_secret_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/secrets");
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

    /// watch individual changes to a list of ServiceAccount

    pub fn watch_core_v1_service_account_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/serviceaccounts");
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

    /// watch individual changes to a list of Service

    pub fn watch_core_v1_service_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/api/v1/watch/services");
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
