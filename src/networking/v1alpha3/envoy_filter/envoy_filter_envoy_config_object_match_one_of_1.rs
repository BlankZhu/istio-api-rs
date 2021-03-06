use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Customizing Envoy configuration generated by Istio.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct EnvoyFilterEnvoyConfigObjectMatchOneOf1 {
    #[serde(rename = "routeConfiguration")]
    pub route_configuration: Box<super::EnvoyFilterRouteConfigurationMatch>,
}

impl EnvoyFilterEnvoyConfigObjectMatchOneOf1 {
    pub fn new(route_configuration: super::EnvoyFilterRouteConfigurationMatch) -> EnvoyFilterEnvoyConfigObjectMatchOneOf1 {
        EnvoyFilterEnvoyConfigObjectMatchOneOf1 {
            route_configuration: Box::new(route_configuration),
        }
    }
}