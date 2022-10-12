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

/// LocalObjectReference : See k8s.io.api.core.v1.LocalObjectReference.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct LocalObjectReference {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LocalObjectReference {
    /// See k8s.io.api.core.v1.LocalObjectReference.
    pub fn new() -> LocalObjectReference {
        LocalObjectReference {
            name: None,
        }
    }
}