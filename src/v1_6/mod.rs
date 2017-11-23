//! Kubernetes remote API - version 1.6.

use hyper;


mod apis;
pub use self::apis::*;

mod apps;
pub use self::apps::*;

mod apps_v1beta1;
pub use self::apps_v1beta1::*;

mod authentication;
pub use self::authentication::*;

mod authentication_v1;
pub use self::authentication_v1::*;

mod authentication_v1beta1;
pub use self::authentication_v1beta1::*;

mod authorization;
pub use self::authorization::*;

mod authorization_v1;
pub use self::authorization_v1::*;

mod authorization_v1beta1;
pub use self::authorization_v1beta1::*;

mod autoscaling;
pub use self::autoscaling::*;

mod autoscaling_v1;
pub use self::autoscaling_v1::*;

mod autoscaling_v2alpha1;
pub use self::autoscaling_v2alpha1::*;

mod batch;
pub use self::batch::*;

mod batch_v1;
pub use self::batch_v1::*;

mod batch_v2alpha1;
pub use self::batch_v2alpha1::*;

mod certificates;
pub use self::certificates::*;

mod certificates_v1beta1;
pub use self::certificates_v1beta1::*;

mod core;
pub use self::core::*;

mod core_v1;
pub use self::core_v1::*;

mod extensions;
pub use self::extensions::*;

mod extensions_v1beta1;
pub use self::extensions_v1beta1::*;

mod generic;
pub use self::generic::*;

mod logs;
pub use self::logs::*;

mod policy;
pub use self::policy::*;

mod policy_v1beta1;
pub use self::policy_v1beta1::*;

mod rbac_authorization;
pub use self::rbac_authorization::*;

mod rbac_authorization_v1alpha1;
pub use self::rbac_authorization_v1alpha1::*;

mod rbac_authorization_v1beta1;
pub use self::rbac_authorization_v1beta1::*;

mod settings;
pub use self::settings::*;

mod settings_v1alpha1;
pub use self::settings_v1alpha1::*;

mod storage;
pub use self::storage::*;

mod storage_v1;
pub use self::storage_v1::*;

mod storage_v1beta1;
pub use self::storage_v1beta1::*;

mod version;
pub use self::version::*;


pub mod types;

/// Build a v1.6 client from configuration.
pub trait BuildClientV16 {
    fn build_v1_6(self) -> Client;
}

impl BuildClientV16 for super::ClientBuilder {
    /// Return a `Client` to interact with the API server.
    fn build_v1_6(self) -> Client {
        let cb = self.build_basic();
        Client(cb)
    }
}

/// Kubernetes client, compatible with API version 1.6.
#[derive(Debug, Clone)]
pub struct Client(super::ClientBasic);

impl Client {
    fn new_request(&self, method: hyper::Method, url: hyper::Uri) -> hyper::client::Request {
        self.0.new_request(method, url)
    }
}

impl From<super::ClientBasic> for Client {
    fn from(cb: super::ClientBasic) -> Self {
        Client(cb)
    }
}

impl From<Client> for super::ClientBasic {
    fn from(c: Client) -> Self {
        c.0
    }
}
