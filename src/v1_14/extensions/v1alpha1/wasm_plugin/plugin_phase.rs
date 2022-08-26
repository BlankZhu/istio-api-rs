use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Extend the functionality provided by the Istio proxy through WebAssembly filters.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PluginPhase : The phase in the filter chain where the plugin will be injected.

/// The phase in the filter chain where the plugin will be injected.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum PluginPhase {
    #[serde(rename = "UNSPECIFIED_PHASE")]
    UNSPECIFIEDPHASE,
    #[serde(rename = "AUTHN")]
    AUTHN,
    #[serde(rename = "AUTHZ")]
    AUTHZ,
    #[serde(rename = "STATS")]
    STATS,

}

impl ToString for PluginPhase {
    fn to_string(&self) -> String {
        match self {
            Self::UNSPECIFIEDPHASE => String::from("UNSPECIFIED_PHASE"),
            Self::AUTHN => String::from("AUTHN"),
            Self::AUTHZ => String::from("AUTHZ"),
            Self::STATS => String::from("STATS"),
        }
    }
}

impl Default for PluginPhase {
    fn default() -> PluginPhase {
        Self::UNSPECIFIEDPHASE
    }
}