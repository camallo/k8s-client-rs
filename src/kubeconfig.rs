//! Types and helpers for `kubeconfig` parsing.

use std::{env, path};
use super::errors::*;

/// Configuration to build a Kubernetes client.
#[derive(Debug, Serialize, Deserialize)]
pub struct KubeConfig {
    pub kind: Option<String>,
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    pub preferences: Preferences,
    pub clusters: Vec<NamedCluster>,
    pub users: Vec<NamedAuthInfo>,
    pub contexts: Vec<NamedContext>,
    #[serde(rename = "current-context")]
    pub current_context: String,
    pub extensions: Option<Vec<NamedExtension>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preferences {
    pub colors: Option<bool>,
    pub extensions: Option<Vec<NamedExtension>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedCluster {
    pub name: String,
    pub cluster: Cluster,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cluster {
    pub server: String,
    #[serde(rename = "insecure-skip-tls-verify")]
    pub insecure_tls: Option<bool>,
    #[serde(rename = "certificate-authority")]
    pub ca: Option<String>,
    #[serde(rename = "certificate-authority-data")]
    pub ca_data: Option<Vec<u8>>,
    pub extensions: Option<Vec<NamedExtension>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedAuthInfo {
    pub name: String,
    pub user: AuthInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    #[serde(rename = "tokenFile")]
    pub token_file: Option<String>,
    #[serde(rename = "client-certificate")]
    pub client_certificate: Option<String>,
    #[serde(rename = "client-key")]
    pub client_key: Option<String>,
    pub impersonate: Option<String>,
    //TODO
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedContext {
    pub name: String,
    pub context: Context,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub cluster: String,
    pub user: String,
    pub namespace: Option<String>,
    pub extensions: Option<Vec<Extension>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedExtension {
    pub name: String,
    pub extension: Extension,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Extension {
    pub extension: String,
}

#[derive(Clone, Debug)]
pub struct ClusterContext {
    pub name: String,
    pub cluster: Cluster,
    pub user: AuthInfo,
    pub namespace: Option<String>,
    pub extensions: Option<Vec<Extension>>,
}

impl KubeConfig {
    pub fn context(&self, name: &str) -> Result<ClusterContext> {
        let ctxs: Vec<&NamedContext> = self.contexts.iter().filter(|c| c.name == name).collect();
        let ctx = match ctxs.len() {
            0 => bail!("unknown context {}", name),
            1 => &ctxs[0].context,
            _ => bail!("ambiguous context {}", name),
        };
        let clus: Vec<&NamedCluster> = self.clusters
            .iter()
            .filter(|c| c.name == ctx.cluster)
            .collect();
        let clu = match clus.len() {
            0 => bail!("unknown cluster {}", name),
            1 => &clus[0].cluster,
            _ => bail!("ambiguous cluster {}", name),
        };
        let auths: Vec<&NamedAuthInfo> = self.users
            .iter()
            .filter(|c| c.name == ctx.cluster)
            .collect();
        let auth = match auths.len() {
            0 => bail!("unknown auth-info {}", name),
            1 => &auths[0].user,
            _ => bail!("ambiguous auth-info {}", name),
        };
        let rc = ClusterContext {
            name: name.to_string(),
            cluster: clu.clone(),
            user: auth.clone(),
            namespace: ctx.namespace.clone(),
            extensions: None,
        };
        Ok(rc)
    }

    pub fn default_context(&self) -> Result<ClusterContext> {
        let dname = self.current_context.as_ref();
        self.context(dname)
    }

    pub fn default_path() -> path::PathBuf {
        env::home_dir()
            .unwrap_or("/root".into())
            .join(".kube")
            .join("config")
    }
}
