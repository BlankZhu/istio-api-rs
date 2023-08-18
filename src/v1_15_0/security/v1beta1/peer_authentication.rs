// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_15_0/security/v1beta1/PeerAuthentication.yaml --api-version v1beta1 -D Default
// kopium version: 0.15.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// PeerAuthentication defines how traffic will be tunneled (or not) to the sidecar.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
#[kube(group = "security.istio.io", version = "v1beta1", kind = "PeerAuthentication", plural = "peerauthentications")]
#[kube(namespaced)]
pub struct PeerAuthenticationSpec {
    /// Mutual TLS settings for workload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtls: Option<PeerAuthenticationMtls>,
    /// Port specific mutual TLS settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portLevelMtls")]
    pub port_level_mtls: Option<BTreeMap<String, PeerAuthenticationPortLevelMtls>>,
    /// The selector determines the workloads to apply the ChannelAuthentication on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<PeerAuthenticationSelector>,
}

/// Mutual TLS settings for workload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct PeerAuthenticationMtls {
    /// Defines the mTLS mode used for peer authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<PeerAuthenticationMtlsMode>,
}

/// Mutual TLS settings for workload.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum PeerAuthenticationMtlsMode {
    #[serde(rename = "UNSET")]
    Unset,
    #[serde(rename = "DISABLE")]
    Disable,
    #[serde(rename = "PERMISSIVE")]
    Permissive,
    #[serde(rename = "STRICT")]
    Strict,
}

/// Port specific mutual TLS settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct PeerAuthenticationPortLevelMtls {
    /// Defines the mTLS mode used for peer authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<PeerAuthenticationPortLevelMtlsMode>,
}

/// Port specific mutual TLS settings.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum PeerAuthenticationPortLevelMtlsMode {
    #[serde(rename = "UNSET")]
    Unset,
    #[serde(rename = "DISABLE")]
    Disable,
    #[serde(rename = "PERMISSIVE")]
    Permissive,
    #[serde(rename = "STRICT")]
    Strict,
}

/// The selector determines the workloads to apply the ChannelAuthentication on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct PeerAuthenticationSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

