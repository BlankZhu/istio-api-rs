use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Describes a collection of workload instances.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ExecHealthCheckConfig {
    /// Command to run. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

impl ExecHealthCheckConfig {
    pub fn new() -> ExecHealthCheckConfig {
        ExecHealthCheckConfig {
            command: None,
        }
    }
}