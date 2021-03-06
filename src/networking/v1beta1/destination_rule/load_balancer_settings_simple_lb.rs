use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting load balancing, outlier detection, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerSettingsSimpleLb : Standard load balancing algorithms that require no tuning.

/// Standard load balancing algorithms that require no tuning.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum LoadBalancerSettingsSimpleLb {
    #[serde(rename = "UNSPECIFIED")]
    UNSPECIFIED,
    #[serde(rename = "LEAST_CONN")]
    LEASTCONN,
    #[serde(rename = "RANDOM")]
    RANDOM,
    #[serde(rename = "PASSTHROUGH")]
    PASSTHROUGH,
    #[serde(rename = "ROUND_ROBIN")]
    ROUNDROBIN,
    #[serde(rename = "LEAST_REQUEST")]
    LEASTREQUEST,

}

impl ToString for LoadBalancerSettingsSimpleLb {
    fn to_string(&self) -> String {
        match self {
            Self::UNSPECIFIED => String::from("UNSPECIFIED"),
            Self::LEASTCONN => String::from("LEAST_CONN"),
            Self::RANDOM => String::from("RANDOM"),
            Self::PASSTHROUGH => String::from("PASSTHROUGH"),
            Self::ROUNDROBIN => String::from("ROUND_ROBIN"),
            Self::LEASTREQUEST => String::from("LEAST_REQUEST"),
        }
    }
}

impl Default for LoadBalancerSettingsSimpleLb {
    fn default() -> LoadBalancerSettingsSimpleLb {
        Self::UNSPECIFIED
    }
}