use futures::{Future, Stream};
use hyper;

use ::StreamBlob;
use ::errors::*;

use std::str::FromStr;

impl super::Client {

    /// get available resources

    pub fn get_batch_v2alpha1_apiresources(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/");
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

    /// list or watch objects of kind CronJob

    pub fn list_batch_v2alpha1_cron_job_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/cronjobs");
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

    /// list or watch objects of kind CronJob

    pub fn list_batch_v2alpha1_namespaced_cron_job(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs");
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

    /// list or watch objects of kind ScheduledJob

    pub fn list_batch_v2alpha1_namespaced_scheduled_job(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs");
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

    /// list or watch objects of kind ScheduledJob

    pub fn list_batch_v2alpha1_scheduled_job_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/scheduledjobs");
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

    /// read the specified CronJob

    pub fn read_batch_v2alpha1_namespaced_cron_job(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs/{name}");
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

    /// read status of the specified CronJob

    pub fn read_batch_v2alpha1_namespaced_cron_job_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/namespaces/{namespace}/cronjobs/{name}/status");
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

    /// read the specified ScheduledJob

    pub fn read_batch_v2alpha1_namespaced_scheduled_job(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs/{name}");
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

    /// read status of the specified ScheduledJob

    pub fn read_batch_v2alpha1_namespaced_scheduled_job_status(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/namespaces/{namespace}/scheduledjobs/{name}/status");
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

    /// watch individual changes to a list of CronJob

    pub fn watch_batch_v2alpha1_cron_job_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/watch/cronjobs");
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

    /// watch changes to an object of kind CronJob

    pub fn watch_batch_v2alpha1_namespaced_cron_job(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/watch/namespaces/{namespace}/cronjobs/{name}");
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

    /// watch individual changes to a list of CronJob

    pub fn watch_batch_v2alpha1_namespaced_cron_job_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/watch/namespaces/{namespace}/cronjobs");
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

    /// watch changes to an object of kind ScheduledJob

    pub fn watch_batch_v2alpha1_namespaced_scheduled_job(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/watch/namespaces/{namespace}/scheduledjobs/{name}");
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

    /// watch individual changes to a list of ScheduledJob

    pub fn watch_batch_v2alpha1_namespaced_scheduled_job_list(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/watch/namespaces/{namespace}/scheduledjobs");
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

    /// watch individual changes to a list of ScheduledJob

    pub fn watch_batch_v2alpha1_scheduled_job_list_for_all_namespaces(&self) -> Result<StreamBlob> {
        let url = {
            let s = format!("{}/{}", self.0.base_url, r"/apis/batch/v2alpha1/watch/scheduledjobs");
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
