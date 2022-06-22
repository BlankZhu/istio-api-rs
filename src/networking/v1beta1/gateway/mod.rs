pub mod port;
pub mod server;
pub mod tls_mode;
pub mod tls_protocol;
pub mod tls_settings;

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[kube(
    group = "networking.istio.io",
    version = "v1beta1",
    kind = "Gateway",
    namespaced
)]
pub struct GatewaySpec {
    /// A list of server specifications.
    #[serde(rename = "servers", skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<server::Server>>,
    /// One or more labels that indicate a specific set of pods/VMs on which this gateway configuration should be applied. By default workloads are searched across all namespaces based on label selectors. This implies that a gateway resource in the namespace \"foo\" can select pods in the namespace \"bar\" based on labels. This behavior can be controlled via the `PILOT_SCOPE_GATEWAY_TO_NAMESPACE` environment variable in istiod. If this variable is set to true, the scope of label search is restricted to the configuration namespace in which the the resource is present. In other words, the Gateway resource must reside in the same namespace as the gateway workload instance. If selector is nil, the Gateway will be applied to all workloads.
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<::std::collections::HashMap<String, String>>,
}

impl GatewaySpec {
    /// Gateway describes a load balancer operating at the edge of the mesh receiving incoming or outgoing HTTP/TCP connections.
    pub fn new() -> GatewaySpec {
        GatewaySpec {
            servers: None,
            selector: None,
        }
    }
}
