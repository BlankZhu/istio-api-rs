use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Extend the functionality provided by the Istio proxy through WebAssembly filters.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct EnvVar {
    /// Required Name of the environment variable. Must be a C_IDENTIFIER.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value for the environment variable. Note that if `value_from` is `HOST`, it will be ignored. Defaults to \"\".
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "valueFrom", skip_serializing_if = "Option::is_none")]
    pub value_from: Option<super::EnvValueSource>,
}

impl EnvVar {
    pub fn new() -> EnvVar {
        EnvVar {
            name: None,
            value: None,
            value_from: None,
        }
    }
}