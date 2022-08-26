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

/// ClientTlsSettingsTlSmode : TLS connection mode

/// TLS connection mode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ClientTlsSettingsTlSmode {
    #[serde(rename = "DISABLE")]
    DISABLE,
    #[serde(rename = "SIMPLE")]
    SIMPLE,
    #[serde(rename = "MUTUAL")]
    MUTUAL,
    #[serde(rename = "ISTIO_MUTUAL")]
    ISTIOMUTUAL,

}

impl ToString for ClientTlsSettingsTlSmode {
    fn to_string(&self) -> String {
        match self {
            Self::DISABLE => String::from("DISABLE"),
            Self::SIMPLE => String::from("SIMPLE"),
            Self::MUTUAL => String::from("MUTUAL"),
            Self::ISTIOMUTUAL => String::from("ISTIO_MUTUAL"),
        }
    }
}

impl Default for ClientTlsSettingsTlSmode {
    fn default() -> ClientTlsSettingsTlSmode {
        Self::DISABLE
    }
}