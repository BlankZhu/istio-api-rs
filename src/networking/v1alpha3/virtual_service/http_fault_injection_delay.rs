use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HttpFaultInjectionDelay : Delay specification is used to inject latency into the request forwarding path. The following example will introduce a 5 second delay in 1 out of every 1000 requests to the \"v1\" version of the \"reviews\" service from all pods with label env: prod



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpFaultInjectionDelay {
    /// Percentage of requests on which the delay will be injected (0-100). Use of integer `percent` value is deprecated. Use the double `percentage` field instead.
    #[serde(rename = "percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<Box<super::Percent>>,
    /// Add a fixed delay before forwarding the request. Format: 1h/1m/1s/1ms. MUST be >=1ms.
    #[serde(rename = "fixedDelay")]
    pub fixed_delay: String,
    #[serde(rename = "exponentialDelay")]
    pub exponential_delay: String,
}

impl HttpFaultInjectionDelay {
    /// Delay specification is used to inject latency into the request forwarding path. The following example will introduce a 5 second delay in 1 out of every 1000 requests to the \"v1\" version of the \"reviews\" service from all pods with label env: prod
    pub fn new(fixed_delay: String, exponential_delay: String) -> HttpFaultInjectionDelay {
        HttpFaultInjectionDelay {
            percent: None,
            percentage: None,
            fixed_delay,
            exponential_delay,
        }
    }
}