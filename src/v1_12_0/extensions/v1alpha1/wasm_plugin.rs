// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_12_0/extensions/v1alpha1/WasmPlugin.yaml --api-version v1alpha1
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "extensions.istio.io", version = "v1alpha1", kind = "WasmPlugin", plural = "wasmplugins")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct WasmPluginSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<WasmPluginImagePullPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecret")]
    pub image_pull_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<WasmPluginPhase>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pluginConfig")]
    pub plugin_config: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pluginName")]
    pub plugin_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<WasmPluginSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verificationKey")]
    pub verification_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WasmPluginImagePullPolicy {
    #[serde(rename = "UNSPECIFIED_POLICY")]
    UnspecifiedPolicy,
    IfNotPresent,
    Always,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WasmPluginPhase {
    #[serde(rename = "UNSPECIFIED_PHASE")]
    UnspecifiedPhase,
    #[serde(rename = "AUTHN")]
    Authn,
    #[serde(rename = "AUTHZ")]
    Authz,
    #[serde(rename = "STATS")]
    Stats,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WasmPluginSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

