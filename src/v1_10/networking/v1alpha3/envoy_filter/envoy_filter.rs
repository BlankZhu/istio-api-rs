use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube::CustomResource;
/*
 * Customizing Envoy configuration generated by Istio.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EnvoyFilterSpec : EnvoyFilter provides a mechanism to customize the Envoy configuration generated by Istio Pilot.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "EnvoyFilter", namespaced)]
pub struct EnvoyFilterSpec {
    #[serde(rename = "workloadSelector", skip_serializing_if = "Option::is_none")]
    pub workload_selector: Option<Box<super::WorkloadSelector>>,
    /// One or more patches with match conditions.
    #[serde(rename = "configPatches", skip_serializing_if = "Option::is_none")]
    pub config_patches: Option<Vec<super::EnvoyFilterEnvoyConfigObjectPatch>>,
}

impl EnvoyFilterSpec {
    /// EnvoyFilter provides a mechanism to customize the Envoy configuration generated by Istio Pilot.
    pub fn new() -> EnvoyFilterSpec {
        EnvoyFilterSpec {
            workload_selector: None,
            config_patches: None,
        }
    }
}