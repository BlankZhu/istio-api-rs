use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Describes a collection of workload instances.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WorkloadEntry : WorkloadEntry enables specifying the properties of a single non-Kubernetes workload such a VM or a bare metal services that can be referred to by service entries.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct WorkloadEntry {
    /// One or more labels associated with the endpoint.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Set of ports associated with the endpoint. If the port map is specified, it must be a map of servicePortName to this endpoint's port, such that traffic to the service port will be forwarded to the endpoint port that maps to the service's portName. If omitted, and the targetPort is specified as part of the service's port specification, traffic to the service port will be forwarded to one of the endpoints on the specified `targetPort`. If both the targetPort and endpoint's port map are not specified, traffic to a service port will be forwarded to one of the endpoints on the same port.
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<::std::collections::HashMap<String, i32>>,
    /// The load balancing weight associated with the endpoint. Endpoints with higher weights will receive proportionally higher traffic.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Address associated with the network endpoint without the port. Domain names can be used if and only if the resolution is set to DNS, and must be fully-qualified without wildcards. Use the form unix:///absolute/path/to/socket for Unix domain socket endpoints.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Network enables Istio to group endpoints resident in the same L3 domain/network. All endpoints in the same network are assumed to be directly reachable from one another. When endpoints in different networks cannot reach each other directly, an Istio Gateway can be used to establish connectivity (usually using the `AUTO_PASSTHROUGH` mode in a Gateway Server). This is an advanced configuration used typically for spanning an Istio mesh over multiple clusters.
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// The locality associated with the endpoint. A locality corresponds to a failure domain (e.g., country/region/zone). Arbitrary failure domain hierarchies can be represented by separating each encapsulating failure domain by /. For example, the locality of an an endpoint in US, in US-East-1 region, within availability zone az-1, in data center rack r11 can be represented as us/us-east-1/az-1/r11. Istio will configure the sidecar to route to endpoints within the same locality as the sidecar. If none of the endpoints in the locality are available, endpoints parent locality (but within the same network ID) will be chosen. For example, if there are two endpoints in same network (networkID \"n1\"), say e1 with locality us/us-east-1/az-1/r11 and e2 with locality us/us-east-1/az-2/r12, a sidecar from us/us-east-1/az-1/r11 locality will prefer e1 from the same locality over e2 from a different locality. Endpoint e2 could be the IP associated with a gateway (that bridges networks n1 and n2), or the IP associated with a standard service endpoint.
    #[serde(rename = "locality", skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// The service account associated with the workload if a sidecar is present in the workload. The service account must be present in the same namespace as the configuration ( WorkloadEntry or a ServiceEntry)
    #[serde(rename = "serviceAccount", skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
}

impl WorkloadEntry {
    /// WorkloadEntry enables specifying the properties of a single non-Kubernetes workload such a VM or a bare metal services that can be referred to by service entries.
    pub fn new() -> WorkloadEntry {
        WorkloadEntry {
            labels: None,
            ports: None,
            weight: None,
            address: None,
            network: None,
            locality: None,
            service_account: None,
        }
    }
}