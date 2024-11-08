// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_10_0/telemetry/v1alpha1/Telemetry.yaml --api-version v1alpha1
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Telemetry defines how the telemetry is generated for workloads within a mesh.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "telemetry.istio.io", version = "v1alpha1", kind = "Telemetry", plural = "telemetries")]
#[kube(namespaced)]
pub struct TelemetrySpec {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<TelemetrySelector>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<Vec<TelemetryTracing>>,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetrySelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracing {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customTags")]
    pub custom_tags: Option<BTreeMap<String, TelemetryTracingCustomTags>>,
    /// Controls span reporting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableSpanReporting")]
    pub disable_span_reporting: Option<bool>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<TelemetryTracingProviders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "randomSamplingPercentage")]
    pub random_sampling_percentage: Option<f64>,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingCustomTags {
    /// Environment adds the value of an environment variable to each span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<TelemetryTracingCustomTagsEnvironment>,
    /// RequestHeader adds the value of an header from the request to each span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<TelemetryTracingCustomTagsHeader>,
    /// Literal adds the same, hard-coded value to each span.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<TelemetryTracingCustomTagsLiteral>,
}

/// Environment adds the value of an environment variable to each span.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingCustomTagsEnvironment {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<String>,
    /// Name of the environment variable from which to extract the tag value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// RequestHeader adds the value of an header from the request to each span.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingCustomTagsHeader {
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<String>,
    /// Name of the header from which to extract the tag value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Literal adds the same, hard-coded value to each span.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingCustomTagsLiteral {
    /// The tag value to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TelemetryTracingProviders {
    /// Required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

