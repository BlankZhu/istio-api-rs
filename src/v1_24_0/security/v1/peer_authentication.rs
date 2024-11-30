// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_24_0/security/v1/PeerAuthentication.yaml --api-version v1
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "security.istio.io", version = "v1", kind = "PeerAuthentication", plural = "peerauthentications")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct PeerAuthenticationSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtls: Option<PeerAuthenticationMtls>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portLevelMtls")]
    pub port_level_mtls: Option<BTreeMap<String, PeerAuthenticationPortLevelMtls>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<PeerAuthenticationSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeerAuthenticationMtls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<PeerAuthenticationMtlsMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeerAuthenticationPortLevelMtls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<PeerAuthenticationPortLevelMtlsMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeerAuthenticationSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeerAuthenticationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<PeerAuthenticationStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeerAuthenticationStatusValidationMessages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<PeerAuthenticationStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<PeerAuthenticationStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PeerAuthenticationStatusValidationMessagesLevel {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "INFO")]
    Info,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeerAuthenticationStatusValidationMessagesType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
