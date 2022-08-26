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

/// Headers : Message headers can be manipulated when Envoy forwards requests to, or responses from, a destination service. Header manipulation rules can be specified for a specific route destination or for all destinations. The following VirtualService adds a `test` header with the value `true` to requests that are routed to any `reviews` service destination. It also removes the `foo` response header, but only from responses coming from the `v1` subset (version) of the `reviews` service.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Headers {
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<super::HeadersHeaderOperations>>,
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<Box<super::HeadersHeaderOperations>>,
}

impl Headers {
    /// Message headers can be manipulated when Envoy forwards requests to, or responses from, a destination service. Header manipulation rules can be specified for a specific route destination or for all destinations. The following VirtualService adds a `test` header with the value `true` to requests that are routed to any `reviews` service destination. It also removes the `foo` response header, but only from responses coming from the `v1` subset (version) of the `reviews` service.
    pub fn new() -> Headers {
        Headers {
            response: None,
            request: None,
        }
    }
}