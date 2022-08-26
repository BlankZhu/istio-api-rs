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




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct LoadBalancerSettingsConsistentHashLbOneOf1 {
    #[serde(rename = "httpCookie")]
    pub http_cookie: Box<super::LoadBalancerSettingsConsistentHashLbHttpCookie>,
}

impl LoadBalancerSettingsConsistentHashLbOneOf1 {
    pub fn new(http_cookie: super::LoadBalancerSettingsConsistentHashLbHttpCookie) -> LoadBalancerSettingsConsistentHashLbOneOf1 {
        LoadBalancerSettingsConsistentHashLbOneOf1 {
            http_cookie: Box::new(http_cookie),
        }
    }
}