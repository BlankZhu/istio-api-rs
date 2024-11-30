// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_24_0/networking/v1/ServiceEntry.yaml --api-version v1
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
#[kube(group = "networking.istio.io", version = "v1", kind = "ServiceEntry", plural = "serviceentries")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct ServiceEntrySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<ServiceEntryEndpoints>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportTo")]
    pub export_to: Option<Vec<String>>,
    pub hosts: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ServiceEntryLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<ServiceEntryPorts>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<ServiceEntryResolution>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<ServiceEntryWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceEntryEndpoints {
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
pub enum ServiceEntryLocation {
    #[serde(rename = "MESH_EXTERNAL")]
    MeshExternal,
    #[serde(rename = "MESH_INTERNAL")]
    MeshInternal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceEntryPorts {
    pub name: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServiceEntryResolution {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "STATIC")]
    Static,
    #[serde(rename = "DNS")]
    Dns,
    #[serde(rename = "DNS_ROUND_ROBIN")]
    DnsRoundRobin,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceEntryWorkloadSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceEntryStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<ServiceEntryStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceEntryStatusValidationMessages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<ServiceEntryStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ServiceEntryStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServiceEntryStatusValidationMessagesLevel {
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
pub struct ServiceEntryStatusValidationMessagesType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

