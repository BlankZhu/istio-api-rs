use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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
pub struct HttpBody {
    /// response body as a string
    #[serde(rename = "string")]
    pub string: String,
    /// response body as base64 encoded bytes.
    #[serde(rename = "bytes")]
    pub bytes: std::path::PathBuf,
}

impl HttpBody {
    pub fn new(string: String, bytes: std::path::PathBuf) -> HttpBody {
        HttpBody {
            string,
            bytes,
        }
    }
}