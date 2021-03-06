use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpRedirectOneOf {
    /// On a redirect, overwrite the port portion of the URL with this value.
    #[serde(rename = "port")]
    pub port: i32,
}

impl HttpRedirectOneOf {
    pub fn new(port: i32) -> HttpRedirectOneOf {
        HttpRedirectOneOf {
            port,
        }
    }
}