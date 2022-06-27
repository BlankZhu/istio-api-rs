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

/// EnvoyFilterPatchOperation : Operation denotes how the patch should be applied to the selected configuration.

/// Operation denotes how the patch should be applied to the selected configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum EnvoyFilterPatchOperation {
    #[serde(rename = "INVALID")]
    INVALID,
    #[serde(rename = "MERGE")]
    MERGE,
    #[serde(rename = "ADD")]
    ADD,
    #[serde(rename = "REMOVE")]
    REMOVE,
    #[serde(rename = "INSERT_BEFORE")]
    INSERTBEFORE,
    #[serde(rename = "INSERT_AFTER")]
    INSERTAFTER,
    #[serde(rename = "INSERT_FIRST")]
    INSERTFIRST,
    #[serde(rename = "REPLACE")]
    REPLACE,

}

impl ToString for EnvoyFilterPatchOperation {
    fn to_string(&self) -> String {
        match self {
            Self::INVALID => String::from("INVALID"),
            Self::MERGE => String::from("MERGE"),
            Self::ADD => String::from("ADD"),
            Self::REMOVE => String::from("REMOVE"),
            Self::INSERTBEFORE => String::from("INSERT_BEFORE"),
            Self::INSERTAFTER => String::from("INSERT_AFTER"),
            Self::INSERTFIRST => String::from("INSERT_FIRST"),
            Self::REPLACE => String::from("REPLACE"),
        }
    }
}

impl Default for EnvoyFilterPatchOperation {
    fn default() -> EnvoyFilterPatchOperation {
        Self::INVALID
    }
}