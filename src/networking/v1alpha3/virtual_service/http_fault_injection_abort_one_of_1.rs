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
pub struct HttpFaultInjectionAbortOneOf1 {
    #[serde(rename = "grpcStatus")]
    pub grpc_status: String,
}

impl HttpFaultInjectionAbortOneOf1 {
    pub fn new(grpc_status: String) -> HttpFaultInjectionAbortOneOf1 {
        HttpFaultInjectionAbortOneOf1 {
            grpc_status,
        }
    }
}