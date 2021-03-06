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

/// OutboundTrafficPolicy : `OutboundTrafficPolicy` sets the default behavior of the sidecar for handling outbound traffic from the application. If your application uses one or more external services that are not known apriori, setting the policy to `ALLOW_ANY` will cause the sidecars to route any unknown traffic originating from the application to its requested destination. Users are strongly encouraged to use `ServiceEntry` configurations to explicitly declare any external dependencies, instead of using `ALLOW_ANY`, so that traffic to these services can be monitored.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct OutboundTrafficPolicy {
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<super::OutboundTrafficPolicyMode>,
    #[serde(rename = "egressProxy", skip_serializing_if = "Option::is_none")]
    pub egress_proxy: Option<Box<super::Destination>>,
}

impl OutboundTrafficPolicy {
    /// `OutboundTrafficPolicy` sets the default behavior of the sidecar for handling outbound traffic from the application. If your application uses one or more external services that are not known apriori, setting the policy to `ALLOW_ANY` will cause the sidecars to route any unknown traffic originating from the application to its requested destination. Users are strongly encouraged to use `ServiceEntry` configurations to explicitly declare any external dependencies, instead of using `ALLOW_ANY`, so that traffic to these services can be monitored.
    pub fn new() -> OutboundTrafficPolicy {
        OutboundTrafficPolicy {
            mode: None,
            egress_proxy: None,
        }
    }
}