// Manual implementation of `version.Info` from
// k8s.io/apimachinery/pkg/version/types.go

use std::fmt;

/// Cluster versioning information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    /// Major version.
    pub major: String,
    /// Minor version, numeric possibly followed by "+".
    pub minor: String,
    /// Semantic version.
    pub git_version: String,
    /// SHA1 from git revision.
    pub git_commit: String,
    /// State of git tree, e.g. "clean" or "dirty".
    pub git_tree_state: String,
    /// Build date, ISO 8601 format.
    pub build_date: String,
    /// Golang runtime version.
    pub go_version: String,
    /// Compiler, as Golang label.
    pub compiler: String,
    /// Platform, as Golang label.
    pub platform: String,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.git_version)
    }
}
