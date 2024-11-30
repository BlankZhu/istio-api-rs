// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_24_0/telemetry/v1alpha1/Telemetry.yaml --api-version v1alpha1
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
#[kube(group = "telemetry.istio.io", version = "v1alpha1", kind = "Telemetry", plural = "telemetries")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct TelemetrySpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessLogging")]
    pub access_logging: Option<Vec<TelemetryAccessLogging>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<TelemetryMetrics>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<TelemetrySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRef")]
    pub target_ref: Option<TelemetryTargetRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRefs")]
    pub target_refs: Option<Vec<TelemetryTargetRefs>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<Vec<TelemetryTracing>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryAccessLogging {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<TelemetryAccessLoggingFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<TelemetryAccessLoggingMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<TelemetryAccessLoggingProviders>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryAccessLoggingFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryAccessLoggingMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TelemetryAccessLoggingMatchMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TelemetryAccessLoggingMatchMode {
    #[serde(rename = "CLIENT_AND_SERVER")]
    ClientAndServer,
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "SERVER")]
    Server,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryAccessLoggingProviders {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryMetrics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<TelemetryMetricsOverrides>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<TelemetryMetricsProviders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reportingInterval")]
    pub reporting_interval: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryMetricsOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<TelemetryMetricsOverridesMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagOverrides")]
    pub tag_overrides: Option<BTreeMap<String, TelemetryMetricsOverridesTagOverrides>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryMetricsOverridesMatch {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customMetric")]
    pub custom_metric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<TelemetryMetricsOverridesMatchMetric>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TelemetryMetricsOverridesMatchMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TelemetryMetricsOverridesMatchMetric {
    #[serde(rename = "ALL_METRICS")]
    AllMetrics,
    #[serde(rename = "REQUEST_COUNT")]
    RequestCount,
    #[serde(rename = "REQUEST_DURATION")]
    RequestDuration,
    #[serde(rename = "REQUEST_SIZE")]
    RequestSize,
    #[serde(rename = "RESPONSE_SIZE")]
    ResponseSize,
    #[serde(rename = "TCP_OPENED_CONNECTIONS")]
    TcpOpenedConnections,
    #[serde(rename = "TCP_CLOSED_CONNECTIONS")]
    TcpClosedConnections,
    #[serde(rename = "TCP_SENT_BYTES")]
    TcpSentBytes,
    #[serde(rename = "TCP_RECEIVED_BYTES")]
    TcpReceivedBytes,
    #[serde(rename = "GRPC_REQUEST_MESSAGES")]
    GrpcRequestMessages,
    #[serde(rename = "GRPC_RESPONSE_MESSAGES")]
    GrpcResponseMessages,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TelemetryMetricsOverridesMatchMode {
    #[serde(rename = "CLIENT_AND_SERVER")]
    ClientAndServer,
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "SERVER")]
    Server,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryMetricsOverridesTagOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<TelemetryMetricsOverridesTagOverridesOperation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TelemetryMetricsOverridesTagOverridesOperation {
    #[serde(rename = "UPSERT")]
    Upsert,
    #[serde(rename = "REMOVE")]
    Remove,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryMetricsProviders {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetrySelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryTargetRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryTargetRefs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryTracing {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customTags")]
    pub custom_tags: Option<BTreeMap<String, TelemetryTracingCustomTags>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableSpanReporting")]
    pub disable_span_reporting: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<TelemetryTracingMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<TelemetryTracingProviders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "randomSamplingPercentage")]
    pub random_sampling_percentage: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useRequestIdForTraceSampling")]
    pub use_request_id_for_trace_sampling: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryTracingCustomTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<TelemetryTracingCustomTagsEnvironment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<TelemetryTracingCustomTagsHeader>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal: Option<TelemetryTracingCustomTagsLiteral>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryTracingCustomTagsEnvironment {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryTracingCustomTagsHeader {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultValue")]
    pub default_value: Option<String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryTracingCustomTagsLiteral {
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryTracingMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TelemetryTracingMatchMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TelemetryTracingMatchMode {
    #[serde(rename = "CLIENT_AND_SERVER")]
    ClientAndServer,
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "SERVER")]
    Server,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryTracingProviders {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<TelemetryStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelemetryStatusValidationMessages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<TelemetryStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<TelemetryStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TelemetryStatusValidationMessagesLevel {
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
pub struct TelemetryStatusValidationMessagesType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
