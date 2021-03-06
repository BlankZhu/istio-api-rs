use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration for access control on workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct AuthorizationPolicyOneOf {
    #[serde(rename = "provider")]
    pub provider: Box<super::AuthorizationPolicyExtensionProvider>,
}

impl AuthorizationPolicyOneOf {
    pub fn new(provider: super::AuthorizationPolicyExtensionProvider) -> AuthorizationPolicyOneOf {
        AuthorizationPolicyOneOf {
            provider: Box::new(provider),
        }
    }
}