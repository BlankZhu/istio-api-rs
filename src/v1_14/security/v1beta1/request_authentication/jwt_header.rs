use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Request authentication configuration for workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JwtHeader : This message specifies a header location to extract JWT token.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct JwtHeader {
    /// The HTTP header name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The prefix that should be stripped before decoding the token. For example, for \"Authorization: Bearer <token>\", prefix=\"Bearer \" with a space at the end. If the header doesn't have this exact prefix, it is considered invalid.
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl JwtHeader {
    /// This message specifies a header location to extract JWT token.
    pub fn new() -> JwtHeader {
        JwtHeader {
            name: None,
            prefix: None,
        }
    }
}