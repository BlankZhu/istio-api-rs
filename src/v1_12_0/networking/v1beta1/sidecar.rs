// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_12_0/networking/v1beta1/Sidecar.yaml --api-version v1beta1
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Configuration affecting network reachability of a sidecar. See more details at: https://istio.io/docs/reference/config/networking/sidecar.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "Sidecar", plural = "sidecars")]
#[kube(namespaced)]
pub struct SidecarSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<SidecarEgress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<SidecarIngress>>,
    /// Configuration for the outbound traffic policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outboundTrafficPolicy")]
    pub outbound_traffic_policy: Option<SidecarOutboundTrafficPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<SidecarWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SidecarEgress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "captureMode")]
    pub capture_mode: Option<SidecarEgressCaptureMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// The port associated with the listener.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarEgressPort>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum SidecarEgressCaptureMode {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "IPTABLES")]
    Iptables,
    #[serde(rename = "NONE")]
    None,
}

/// The port associated with the listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SidecarEgressPort {
    /// Label assigned to the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A valid non-negative integer port number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    /// The protocol exposed on the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SidecarIngress {
    /// The IP to which the listener should be bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "captureMode")]
    pub capture_mode: Option<SidecarIngressCaptureMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultEndpoint")]
    pub default_endpoint: Option<String>,
    /// The port associated with the listener.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarIngressPort>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum SidecarIngressCaptureMode {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "IPTABLES")]
    Iptables,
    #[serde(rename = "NONE")]
    None,
}

/// The port associated with the listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SidecarIngressPort {
    /// Label assigned to the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A valid non-negative integer port number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    /// The protocol exposed on the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

/// Configuration for the outbound traffic policy.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SidecarOutboundTrafficPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "egressProxy")]
    pub egress_proxy: Option<SidecarOutboundTrafficPolicyEgressProxy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<SidecarOutboundTrafficPolicyMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SidecarOutboundTrafficPolicyEgressProxy {
    /// The name of a service from the service registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarOutboundTrafficPolicyEgressProxyPort>,
    /// The name of a subset within the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

/// Specifies the port on the host that is being addressed.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SidecarOutboundTrafficPolicyEgressProxyPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

/// Configuration for the outbound traffic policy.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum SidecarOutboundTrafficPolicyMode {
    #[serde(rename = "REGISTRY_ONLY")]
    RegistryOnly,
    #[serde(rename = "ALLOW_ANY")]
    AllowAny,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SidecarWorkloadSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

