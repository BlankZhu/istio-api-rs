// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_14_0/networking/v1alpha3/ServiceEntry.yaml --api-version v1alpha3
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "ServiceEntry", plural = "serviceentries")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct ServiceEntrySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<ServiceEntryEndpoints>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportTo")]
    pub export_to: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
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

