use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting Istio control plane installation version and shape.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServiceSpec : See k8s.io.api.core.v1.ServiceSpec.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ServiceSpec {
    #[serde(rename = "clusterIP", skip_serializing_if = "Option::is_none")]
    pub cluster_ip: Option<String>,
    #[serde(rename = "externalIPs", skip_serializing_if = "Option::is_none")]
    pub external_ips: Option<Vec<String>>,
    #[serde(rename = "externalName", skip_serializing_if = "Option::is_none")]
    pub external_name: Option<String>,
    #[serde(rename = "externalTrafficPolicy", skip_serializing_if = "Option::is_none")]
    pub external_traffic_policy: Option<String>,
    #[serde(rename = "healthCheckNodePort", skip_serializing_if = "Option::is_none")]
    pub health_check_node_port: Option<i32>,
    #[serde(rename = "loadBalancerIP", skip_serializing_if = "Option::is_none")]
    pub load_balancer_ip: Option<String>,
    #[serde(rename = "loadBalancerSourceRanges", skip_serializing_if = "Option::is_none")]
    pub load_balancer_source_ranges: Option<Vec<String>>,
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<super::ServicePort>>,
    #[serde(rename = "publishNotReadyAddresses", skip_serializing_if = "Option::is_none")]
    pub publish_not_ready_addresses: Option<bool>,
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "sessionAffinity", skip_serializing_if = "Option::is_none")]
    pub session_affinity: Option<String>,
    #[serde(rename = "sessionAffinityConfig", skip_serializing_if = "Option::is_none")]
    pub session_affinity_config: Option<Box<super::SessionAffinityConfig>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl ServiceSpec {
    /// See k8s.io.api.core.v1.ServiceSpec.
    pub fn new() -> ServiceSpec {
        ServiceSpec {
            cluster_ip: None,
            external_ips: None,
            external_name: None,
            external_traffic_policy: None,
            health_check_node_port: None,
            load_balancer_ip: None,
            load_balancer_source_ranges: None,
            ports: None,
            publish_not_ready_addresses: None,
            selector: None,
            session_affinity: None,
            session_affinity_config: None,
            _type: None,
        }
    }
}