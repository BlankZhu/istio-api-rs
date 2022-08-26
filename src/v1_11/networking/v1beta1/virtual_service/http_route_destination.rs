use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HttpRouteDestination : Each routing rule is associated with one or more service versions (see glossary in beginning of document). Weights associated with the version determine the proportion of traffic it receives. For example, the following rule will route 25% of traffic for the \"reviews\" service to instances with the \"v2\" tag and the remaining traffic (i.e., 75%) to \"v1\".



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpRouteDestination {
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<super::Destination>>,
    /// The proportion of traffic to be forwarded to the service version. (0-100). Sum of weights across destinations SHOULD BE == 100. If there is only one destination in a rule, the weight value is assumed to be 100.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Box<super::Headers>>,
}

impl HttpRouteDestination {
    /// Each routing rule is associated with one or more service versions (see glossary in beginning of document). Weights associated with the version determine the proportion of traffic it receives. For example, the following rule will route 25% of traffic for the \"reviews\" service to instances with the \"v2\" tag and the remaining traffic (i.e., 75%) to \"v1\".
    pub fn new() -> HttpRouteDestination {
        HttpRouteDestination {
            destination: None,
            weight: None,
            headers: None,
        }
    }
}