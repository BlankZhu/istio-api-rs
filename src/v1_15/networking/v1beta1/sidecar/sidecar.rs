use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting network reachability of a sidecar.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SidecarSpec : `Sidecar` describes the configuration of the sidecar proxy that mediates inbound and outbound communication of the workload instance to which it is attached.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "Sidecar", namespaced)]
pub struct SidecarSpec {
    /// Egress specifies the configuration of the sidecar for processing outbound traffic from the attached workload instance to other services in the mesh. If not specified, inherits the system detected defaults from the namespace-wide or the global default Sidecar.
    #[serde(rename = "egress", skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<super::IstioEgressListener>>,
    /// Ingress specifies the configuration of the sidecar for processing inbound traffic to the attached workload instance. If omitted, Istio will automatically configure the sidecar based on the information about the workload obtained from the orchestration platform (e.g., exposed ports, services, etc.). If specified, inbound ports are configured if and only if the workload instance is associated with a service.
    #[serde(rename = "ingress", skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<super::IstioIngressListener>>,
    #[serde(rename = "outboundTrafficPolicy", skip_serializing_if = "Option::is_none")]
    pub outbound_traffic_policy: Option<Box<super::OutboundTrafficPolicy>>,
    #[serde(rename = "workloadSelector", skip_serializing_if = "Option::is_none")]
    pub workload_selector: Option<Box<crate::r#type::v1beta1::selector::workload_selector::WorkloadSelector>>,
}

impl SidecarSpec {
    /// `Sidecar` describes the configuration of the sidecar proxy that mediates inbound and outbound communication of the workload instance to which it is attached.
    pub fn new() -> SidecarSpec {
        SidecarSpec {
            egress: None,
            ingress: None,
            outbound_traffic_policy: None,
            workload_selector: None,
        }
    }
}