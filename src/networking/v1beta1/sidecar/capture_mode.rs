use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting network reachability of a sidecar.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CaptureMode : `CaptureMode` describes how traffic to a listener is expected to be captured. Applicable only when the listener is bound to an IP.

/// `CaptureMode` describes how traffic to a listener is expected to be captured. Applicable only when the listener is bound to an IP.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum CaptureMode {
    #[serde(rename = "DEFAULT")]
    DEFAULT,
    #[serde(rename = "IPTABLES")]
    IPTABLES,
    #[serde(rename = "NONE")]
    NONE,

}

impl ToString for CaptureMode {
    fn to_string(&self) -> String {
        match self {
            Self::DEFAULT => String::from("DEFAULT"),
            Self::IPTABLES => String::from("IPTABLES"),
            Self::NONE => String::from("NONE"),
        }
    }
}

impl Default for CaptureMode {
    fn default() -> CaptureMode {
        Self::DEFAULT
    }
}