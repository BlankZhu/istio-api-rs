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

/// MetricValueStatus : See k8s.io.autoscaling.v2beta2.MetricValueStatus.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct MetricValueStatus {
    #[serde(rename = "averageUtilization", skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
    #[serde(rename = "averageValue", skip_serializing_if = "Option::is_none")]
    pub average_value: Option<Box<super::IntOrString>>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<super::IntOrString>>,
}

impl MetricValueStatus {
    /// See k8s.io.autoscaling.v2beta2.MetricValueStatus.
    pub fn new() -> MetricValueStatus {
        MetricValueStatus {
            average_utilization: None,
            average_value: None,
            value: None,
        }
    }
}