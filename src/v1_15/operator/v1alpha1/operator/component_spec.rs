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

/// ComponentSpec : Configuration for internal components.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ComponentSpec {
    /// Selects whether this component is installed.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Hub for the component (overrides top level hub setting).
    #[serde(rename = "hub", skip_serializing_if = "Option::is_none")]
    pub hub: Option<String>,
    #[serde(rename = "k8s", skip_serializing_if = "Option::is_none")]
    pub k8s: Option<Box<super::KubernetesResourcesSpec>>,
    /// Namespace for the component.
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Arbitrary install time configuration for the component.
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<serde_json::Value>,
    /// Tag for the component (overrides top level tag setting).
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<serde_json::Value>,
}

impl ComponentSpec {
    /// Configuration for internal components.
    pub fn new() -> ComponentSpec {
        ComponentSpec {
            enabled: None,
            hub: None,
            k8s: None,
            namespace: None,
            spec: None,
            tag: None,
        }
    }
}