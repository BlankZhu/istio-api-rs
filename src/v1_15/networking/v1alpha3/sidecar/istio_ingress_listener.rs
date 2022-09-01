use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting network reachability of a sidecar.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IstioIngressListener : `IstioIngressListener` specifies the properties of an inbound traffic listener on the sidecar proxy attached to a workload instance.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct IstioIngressListener {
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<Box<super::Port>>,
    /// The IP(IPv4 or IPv6) to which the listener should be bound. Unix domain socket addresses are not allowed in the bind field for ingress listeners. If omitted, Istio will automatically configure the defaults based on imported services and the workload instances to which this configuration is applied to.
    #[serde(rename = "bind", skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(rename = "captureMode", skip_serializing_if = "Option::is_none")]
    pub capture_mode: Option<super::CaptureMode>,
    /// The IP endpoint or Unix domain socket to which traffic should be forwarded to. This configuration can be used to redirect traffic arriving at the bind `IP:Port` on the sidecar to a `localhost:port` or Unix domain socket where the application workload instance is listening for connections. Arbitrary IPs are not supported. Format should be one of `127.0.0.1:PORT`, `[::1]:PORT` (forward to localhost), `0.0.0.0:PORT`, `[::]:PORT` (forward to the instance IP), or `unix:///path/to/socket` (forward to Unix domain socket).
    #[serde(rename = "defaultEndpoint", skip_serializing_if = "Option::is_none")]
    pub default_endpoint: Option<String>,
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Box<super::ServerTlsSettings>>,
}

impl IstioIngressListener {
    /// `IstioIngressListener` specifies the properties of an inbound traffic listener on the sidecar proxy attached to a workload instance.
    pub fn new() -> IstioIngressListener {
        IstioIngressListener {
            port: None,
            bind: None,
            capture_mode: None,
            default_endpoint: None,
            tls: None,
        }
    }
}