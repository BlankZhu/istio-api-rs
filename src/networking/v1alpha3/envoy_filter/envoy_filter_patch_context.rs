use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Customizing Envoy configuration generated by Istio.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnvoyFilterPatchContext : PatchContext selects a class of configurations based on the traffic flow direction and workload type.

/// PatchContext selects a class of configurations based on the traffic flow direction and workload type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum EnvoyFilterPatchContext {
    #[serde(rename = "ANY")]
    ANY,
    #[serde(rename = "SIDECAR_INBOUND")]
    SIDECARINBOUND,
    #[serde(rename = "SIDECAR_OUTBOUND")]
    SIDECAROUTBOUND,
    #[serde(rename = "GATEWAY")]
    GATEWAY,

}

impl ToString for EnvoyFilterPatchContext {
    fn to_string(&self) -> String {
        match self {
            Self::ANY => String::from("ANY"),
            Self::SIDECARINBOUND => String::from("SIDECAR_INBOUND"),
            Self::SIDECAROUTBOUND => String::from("SIDECAR_OUTBOUND"),
            Self::GATEWAY => String::from("GATEWAY"),
        }
    }
}

impl Default for EnvoyFilterPatchContext {
    fn default() -> EnvoyFilterPatchContext {
        Self::ANY
    }
}