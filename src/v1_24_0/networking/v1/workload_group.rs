// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_24_0/networking/v1/WorkloadGroup.yaml --api-version v1
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
#[kube(group = "networking.istio.io", version = "v1", kind = "WorkloadGroup", plural = "workloadgroups")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct WorkloadGroupSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<WorkloadGroupMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe: Option<WorkloadGroupProbe>,
    pub template: WorkloadGroupTemplate,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadGroupMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadGroupProbe {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<WorkloadGroupProbeExec>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureThreshold")]
    pub failure_threshold: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpGet")]
    pub http_get: Option<WorkloadGroupProbeHttpGet>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initialDelaySeconds")]
    pub initial_delay_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "periodSeconds")]
    pub period_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successThreshold")]
    pub success_threshold: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpSocket")]
    pub tcp_socket: Option<WorkloadGroupProbeTcpSocket>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadGroupProbeExec {
    pub command: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadGroupProbeHttpGet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpHeaders")]
    pub http_headers: Option<Vec<WorkloadGroupProbeHttpGetHttpHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub port: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadGroupProbeHttpGetHttpHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadGroupProbeTcpSocket {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    pub port: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadGroupTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<BTreeMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadGroupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<WorkloadGroupStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkloadGroupStatusValidationMessages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<WorkloadGroupStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<WorkloadGroupStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WorkloadGroupStatusValidationMessagesLevel {
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
pub struct WorkloadGroupStatusValidationMessagesType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
