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

/// IstioEgressListener : `IstioEgressListener` specifies the properties of an outbound traffic listener on the sidecar proxy attached to a workload instance.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct IstioEgressListener {
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<Box<super::Port>>,
    /// The IP or the Unix domain socket to which the listener should be bound to. Port MUST be specified if bind is not empty. Format: `x.x.x.x` or `unix:///path/to/uds` or `unix://@foobar` (Linux abstract namespace). If omitted, Istio will automatically configure the defaults based on imported services, the workload instances to which this configuration is applied to and the captureMode. If captureMode is `NONE`, bind will default to 127.0.0.1.
    #[serde(rename = "bind", skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(rename = "captureMode", skip_serializing_if = "Option::is_none")]
    pub capture_mode: Option<super::CaptureMode>,
    /// One or more service hosts exposed by the listener in `namespace/dnsName` format. Services in the specified namespace matching `dnsName` will be exposed. The corresponding service can be a service in the service registry (e.g., a Kubernetes or cloud foundry service) or a service specified using a `ServiceEntry` or `VirtualService` configuration. Any associated `DestinationRule` in the same namespace will also be used.
    #[serde(rename = "hosts", skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
}

impl IstioEgressListener {
    /// `IstioEgressListener` specifies the properties of an outbound traffic listener on the sidecar proxy attached to a workload instance.
    pub fn new() -> IstioEgressListener {
        IstioEgressListener {
            port: None,
            bind: None,
            capture_mode: None,
            hosts: None,
        }
    }
}