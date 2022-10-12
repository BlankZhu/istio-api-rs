use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting Istio control plane installation version and shape.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstallStatusStatus : Status describes the current state of a component.

/// Status describes the current state of a component.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum InstallStatusStatus {
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "UPDATING")]
    UPDATING,
    #[serde(rename = "RECONCILING")]
    RECONCILING,
    #[serde(rename = "HEALTHY")]
    HEALTHY,
    #[serde(rename = "ERROR")]
    ERROR,
    #[serde(rename = "ACTION_REQUIRED")]
    ACTIONREQUIRED,

}

impl ToString for InstallStatusStatus {
    fn to_string(&self) -> String {
        match self {
            Self::NONE => String::from("NONE"),
            Self::UPDATING => String::from("UPDATING"),
            Self::RECONCILING => String::from("RECONCILING"),
            Self::HEALTHY => String::from("HEALTHY"),
            Self::ERROR => String::from("ERROR"),
            Self::ACTIONREQUIRED => String::from("ACTION_REQUIRED"),
        }
    }
}

impl Default for InstallStatusStatus {
    fn default() -> InstallStatusStatus {
        Self::NONE
    }
}