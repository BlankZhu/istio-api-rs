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

/// Resources : See k8s.io.api.core.v1.ResourceRequirements.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Resources {
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "requests", skip_serializing_if = "Option::is_none")]
    pub requests: Option<::std::collections::HashMap<String, String>>,
}

impl Resources {
    /// See k8s.io.api.core.v1.ResourceRequirements.
    pub fn new() -> Resources {
        Resources {
            limits: None,
            requests: None,
        }
    }
}