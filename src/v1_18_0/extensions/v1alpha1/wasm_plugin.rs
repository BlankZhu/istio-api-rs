// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_18_0/extensions/v1alpha1/WasmPlugin.yaml --api-version v1alpha1 -D Default
// kopium version: 0.15.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Extend the functionality provided by the Istio proxy through WebAssembly filters. See more details at: https://istio.io/docs/reference/config/proxy_extensions/wasm-plugin.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
#[kube(group = "extensions.istio.io", version = "v1alpha1", kind = "WasmPlugin", plural = "wasmplugins")]
#[kube(namespaced)]
pub struct WasmPluginSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<WasmPluginImagePullPolicy>,
    /// Credentials to use for OCI image pulling.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecret")]
    pub image_pull_secret: Option<String>,
    /// Specifies the criteria to determine which traffic is passed to WasmPlugin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<WasmPluginMatch>>,
    /// Determines where in the filter chain this `WasmPlugin` is to be injected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<WasmPluginPhase>,
    /// The configuration that will be passed on to the plugin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pluginConfig")]
    pub plugin_config: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pluginName")]
    pub plugin_name: Option<String>,
    /// Determines ordering of `WasmPlugins` in the same `phase`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<WasmPluginSelector>,
    /// SHA256 checksum that will be used to verify Wasm module or OCI container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// URL of a Wasm module or OCI container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verificationKey")]
    pub verification_key: Option<String>,
    /// Configuration for a Wasm VM.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vmConfig")]
    pub vm_config: Option<WasmPluginVmConfig>,
}

/// Extend the functionality provided by the Istio proxy through WebAssembly filters. See more details at: https://istio.io/docs/reference/config/proxy_extensions/wasm-plugin.html
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum WasmPluginImagePullPolicy {
    #[serde(rename = "UNSPECIFIED_POLICY")]
    UnspecifiedPolicy,
    IfNotPresent,
    Always,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WasmPluginMatch {
    /// Criteria for selecting traffic by their direction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<WasmPluginMatchMode>,
    /// Criteria for selecting traffic by their destination port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<WasmPluginMatchPorts>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum WasmPluginMatchMode {
    #[serde(rename = "UNDEFINED")]
    Undefined,
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "SERVER")]
    Server,
    #[serde(rename = "CLIENT_AND_SERVER")]
    ClientAndServer,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WasmPluginMatchPorts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

/// Extend the functionality provided by the Istio proxy through WebAssembly filters. See more details at: https://istio.io/docs/reference/config/proxy_extensions/wasm-plugin.html
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WasmPluginSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// Configuration for a Wasm VM.
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WasmPluginVmConfig {
    /// Specifies environment variables to be injected to this VM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<WasmPluginVmConfigEnv>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema)]
pub struct WasmPluginVmConfigEnv {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value for the environment variable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<WasmPluginVmConfigEnvValueFrom>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum WasmPluginVmConfigEnvValueFrom {
    #[serde(rename = "INLINE")]
    Inline,
    #[serde(rename = "HOST")]
    Host,
}

