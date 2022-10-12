use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Customizing Envoy configuration generated by Istio.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnvoyFilterSpec : EnvoyFilterSpec provides a mechanism to customize the Envoy configuration generated by Istio Pilot.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "EnvoyFilter", namespaced)]
pub struct EnvoyFilterSpec {
    /// One or more patches with match conditions.
    #[serde(rename = "configPatches", skip_serializing_if = "Option::is_none")]
    pub config_patches: Option<Vec<super::EnvoyFilterEnvoyConfigObjectPatch>>,
    /// Priority defines the order in which patch sets are applied within a context. When one patch depends on another patch, the order of patch application is significant. The API provides two primary ways to order patches. Patch sets in the root namespace are applied before the patch sets in the workload namespace. Patches within a patch set are processed in the order that they appear in the `configPatches` list.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "workloadSelector", skip_serializing_if = "Option::is_none")]
    pub workload_selector: Option<Box<crate::r#type::v1beta1::selector::workload_selector::WorkloadSelector>>,
}

impl EnvoyFilterSpec {
    /// EnvoyFilterSpec provides a mechanism to customize the Envoy configuration generated by Istio Pilot.
    pub fn new() -> EnvoyFilterSpec {
        EnvoyFilterSpec {
            config_patches: None,
            priority: None,
            workload_selector: None,
        }
    }
}