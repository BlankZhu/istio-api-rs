use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube::CustomResource;
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VirtualServiceSpec : Configuration affecting traffic routing.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "VirtualService", namespaced)]
pub struct VirtualServiceSpec {
    /// A list of namespaces to which this virtual service is exported. Exporting a virtual service allows it to be used by sidecars and gateways defined in other namespaces. This feature provides a mechanism for service owners and mesh administrators to control the visibility of virtual services across namespace boundaries.
    #[serde(rename = "exportTo", skip_serializing_if = "Option::is_none")]
    pub export_to: Option<Vec<String>>,
    /// An ordered list of route rule for non-terminated TLS & HTTPS traffic. Routing is typically performed using the SNI value presented by the ClientHello message. TLS routes will be applied to platform service ports named 'https-*', 'tls-*', unterminated gateway ports using HTTPS/TLS protocols (i.e. with \"passthrough\" TLS mode) and service entry ports using HTTPS/TLS protocols. The first rule matching an incoming request is used. NOTE: Traffic 'https-*' or 'tls-*' ports without associated virtual service will be treated as opaque TCP traffic.
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Vec<super::TlsRoute>>,
    /// An ordered list of route rules for opaque TCP traffic. TCP routes will be applied to any port that is not a HTTP or TLS port. The first rule matching an incoming request is used.
    #[serde(rename = "tcp", skip_serializing_if = "Option::is_none")]
    pub tcp: Option<Vec<super::TcpRoute>>,
    /// An ordered list of route rules for HTTP traffic. HTTP routes will be applied to platform service ports named 'http-*'/'http2-*'/'grpc-*', gateway ports with protocol HTTP/HTTP2/GRPC/ TLS-terminated-HTTPS and service entry ports using HTTP/HTTP2/GRPC protocols. The first rule matching an incoming request is used.
    #[serde(rename = "http", skip_serializing_if = "Option::is_none")]
    pub http: Option<Vec<super::HttpRoute>>,
    /// The destination hosts to which traffic is being sent. Could be a DNS name with wildcard prefix or an IP address. Depending on the platform, short-names can also be used instead of a FQDN (i.e. has no dots in the name). In such a scenario, the FQDN of the host would be derived based on the underlying platform.
    #[serde(rename = "hosts", skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// The names of gateways and sidecars that should apply these routes. Gateways in other namespaces may be referred to by `<gateway namespace>/<gateway name>`; specifying a gateway with no namespace qualifier is the same as specifying the VirtualService's namespace. A single VirtualService is used for sidecars inside the mesh as well as for one or more gateways. The selection condition imposed by this field can be overridden using the source field in the match conditions of protocol-specific routes. The reserved word `mesh` is used to imply all the sidecars in the mesh. When this field is omitted, the default gateway (`mesh`) will be used, which would apply the rule to all sidecars in the mesh. If a list of gateway names is provided, the rules will apply only to the gateways. To apply the rules to both gateways and sidecars, specify `mesh` as one of the gateway names.
    #[serde(rename = "gateways", skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
}

impl VirtualServiceSpec {
    /// Configuration affecting traffic routing.
    pub fn new() -> VirtualServiceSpec {
        VirtualServiceSpec {
            export_to: None,
            tls: None,
            tcp: None,
            http: None,
            hosts: None,
            gateways: None,
        }
    }
}