use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube::CustomResource;
/*
 * Provides configuration for individual workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ProxyConfigSpec : `ProxyConfig` exposes proxy level configuration options.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(group = "networking.istio.io", version = "v1beta1", kind = "ProxyConfig", namespaced)]
pub struct ProxyConfigSpec {
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<Box<crate::r#type::v1beta1::workload_selector::WorkloadSelector>>,
    /// The number of worker threads to run. If unset, defaults to 2. If set to 0, this will be configured to use all cores on the machine using CPU requests and limits to choose a value, with limits taking precedence over requests.
    #[serde(rename = "concurrency", skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i32>,
    /// Additional environment variables for the proxy. Names starting with `ISTIO_META_` will be included in the generated bootstrap configuration and sent to the XDS server.
    #[serde(rename = "environmentVariables", skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<super::ProxyImage>>,
}

impl ProxyConfigSpec {
    /// `ProxyConfig` exposes proxy level configuration options.
    pub fn new() -> ProxyConfigSpec {
        ProxyConfigSpec {
            selector: None,
            concurrency: None,
            environment_variables: None,
            image: None,
        }
    }
}