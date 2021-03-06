use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube::CustomResource;
/*
 * Configuration for access control on workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthorizationPolicySpec : AuthorizationPolicy enables access control on workloads.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(group = "security.istio.io", version = "v1beta1", kind = "AuthorizationPolicy", namespaced)]
pub struct AuthorizationPolicySpec {
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<Box<crate::r#type::v1beta1::workload_selector::WorkloadSelector>>,
    /// Optional. A list of rules to match the request. A match occurs when at least one rule matches the request.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<super::Rule>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<super::AuthorizationPolicyAction>,
    #[serde(rename = "provider")]
    pub provider: Box<super::AuthorizationPolicyExtensionProvider>,
}

impl AuthorizationPolicySpec {
    /// AuthorizationPolicy enables access control on workloads.
    pub fn new(provider: super::AuthorizationPolicyExtensionProvider) -> AuthorizationPolicySpec {
        AuthorizationPolicySpec {
            selector: None,
            rules: None,
            action: None,
            provider: Box::new(provider),
        }
    }
}