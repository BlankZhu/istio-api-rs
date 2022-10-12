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

/// ResourceMetricStatus : See k8s.io.autoscaling.v2beta2.ResourceMetricStatus.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ResourceMetricStatus {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<Box<super::MetricValueStatus>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ResourceMetricStatus {
    /// See k8s.io.autoscaling.v2beta2.ResourceMetricStatus.
    pub fn new() -> ResourceMetricStatus {
        ResourceMetricStatus {
            current: None,
            name: None,
        }
    }
}