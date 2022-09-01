use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting label/content routing, sni routing, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpBodyOneOf {
    /// response body as a string
    #[serde(rename = "string")]
    pub string: String,
}

impl HttpBodyOneOf {
    pub fn new(string: String) -> HttpBodyOneOf {
        HttpBodyOneOf {
            string,
        }
    }
}