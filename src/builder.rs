use hyper;
use hyper_rustls;
use rustls;
use std::{borrow, fs, io};
use tokio_core::reactor;

use super::errors::*;
use super::kubeconfig;
use super::client_basic::*;

/// Client Builder.
///
/// Build a Client via this configuration struct.
pub struct ClientBuilder {
    pub(crate) http_config: hyper::client::Config<hyper_rustls::HttpsConnector, hyper::Body>,
    pub(crate) handle: reactor::Handle,
    pub(crate) server: String,
    pub(crate) tls_config: hyper_rustls::HttpsConnector,
}

impl ClientBuilder {
    /// Initialize `Config` with default values.
    pub fn new(handle: &reactor::Handle) -> Self {
        Self {
            http_config: hyper::client::Client::configure().connector(
                hyper_rustls::HttpsConnector::new(4, handle),
            ),
            server: "https://localhost:8443".to_owned(),
            handle: handle.clone(),
            tls_config: hyper_rustls::HttpsConnector::new(4, handle),
        }
    }

    // Initialize `ClientBuilder` from a `KubeContext`.
    //
    // Initialization may fail, e.g. if key/cert files are missing.
    pub fn try_from(args: (kubeconfig::ClusterContext, reactor::Handle)) -> Result<Self> {
        let mut cc = Self::new(&args.1);
        cc.server = args.0.cluster.server;
        let mut tls = rustls::ClientConfig::new();
        if let Some(ca) = args.0.cluster.ca {
            let f = fs::File::open(&ca)?;
            let mut rd = io::BufReader::new(f);
            tls.root_store.add_pem_file(&mut rd).map_err(|_| {
                rustls::TLSError::General(ca)
            })?;
        };
        if let Some(keyfile) = args.0.user.client_key {
            let kf = fs::File::open(&keyfile)?;
            let mut krd = io::BufReader::new(kf);
            let keys = rustls::internal::pemfile::rsa_private_keys(&mut krd).map_err(|_| {
                rustls::TLSError::General(keyfile.clone())
            })?;
            let key = keys.get(0).ok_or(rustls::TLSError::General(keyfile))?;
            let certfile = args.0.user.client_certificate.unwrap_or_default();
            let cf = fs::File::open(&certfile)?;
            let mut crd = io::BufReader::new(cf);
            let certs = rustls::internal::pemfile::certs(&mut crd).map_err(|_| {
                rustls::TLSError::General(certfile)
            })?;
            tls.set_single_client_cert(certs, key.clone());
        }
        let https = {
            let mut http = hyper::client::HttpConnector::new(4, &cc.handle);
            http.enforce_http(false);
            hyper_rustls::HttpsConnector::from((http, tls))
        };
        cc = cc.tls_config(https);
        Ok(cc)
    }

    /// Set the host to be used to reach the API server.
    pub fn server<S>(mut self, url: S) -> Self
    where
        S: Into<borrow::Cow<'static, str>>,
    {
        self.server = url.into().into_owned();
        self
    }

    /// Set the TLS config to be used to reach the API server.
    pub fn tls_config(mut self, cfg: hyper_rustls::HttpsConnector) -> Self {
        self.tls_config = cfg;
        self
    }

    /// Build a basic client.
    pub fn build_basic(self) -> ClientBasic {
        let hclient = self.http_config.connector(self.tls_config).build(
            &self.handle,
        );
        let hostname = self.server
            .trim_left_matches("https://")
            .split(":")
            .take(1)
            .collect();
        trace!("Built basic client - endpoint {:?}", self.server);
        let c = ClientBasic {
            base_url: self.server,
            hclient: hclient,
            hostname: hostname,
        };
        c
    }
}
