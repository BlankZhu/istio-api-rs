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

/// HttpMatchRequest : HttpMatchRequest specifies a set of criterion to be met in order for the rule to be applied to the HTTP request. For example, the following restricts the rule to match only requests where the URL path starts with /ratings/v2/ and the request contains a custom `end-user` header with value `jason`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct HttpMatchRequest {
    /// The name assigned to a match. The match's name will be concatenated with the parent route's name and will be logged in the access logs for requests matching this route.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<Box<super::StringMatch>>,
    /// Specifies the ports on the host that is being addressed. Many services only expose a single port or label ports with the protocols they support, in these cases it is not required to explicitly select the port.
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Names of gateways where the rule should be applied. Gateway names in the top-level `gateways` field of the VirtualService (if any) are overridden. The gateway match is independent of sourceLabels.
    #[serde(rename = "gateways", skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    /// The header keys must be lowercase and use hyphen as the separator, e.g. _x-request-id_.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, super::StringMatch>>,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Box<super::StringMatch>>,
    #[serde(rename = "scheme", skip_serializing_if = "Option::is_none")]
    pub scheme: Option<Box<super::StringMatch>>,
    #[serde(rename = "authority", skip_serializing_if = "Option::is_none")]
    pub authority: Option<Box<super::StringMatch>>,
    /// One or more labels that constrain the applicability of a rule to workloads with the given labels. If the VirtualService has a list of gateways specified in the top-level `gateways` field, it must include the reserved gateway `mesh` for this field to be applicable.
    #[serde(rename = "sourceLabels", skip_serializing_if = "Option::is_none")]
    pub source_labels: Option<::std::collections::HashMap<String, String>>,
    /// Query parameters for matching.
    #[serde(rename = "queryParams", skip_serializing_if = "Option::is_none")]
    pub query_params: Option<::std::collections::HashMap<String, super::StringMatch>>,
    /// Flag to specify whether the URI matching should be case-insensitive.
    #[serde(rename = "ignoreUriCase", skip_serializing_if = "Option::is_none")]
    pub ignore_uri_case: Option<bool>,
    /// withoutHeader has the same syntax with the header, but has opposite meaning. If a header is matched with a matching rule among withoutHeader, the traffic becomes not matched one.
    #[serde(rename = "withoutHeaders", skip_serializing_if = "Option::is_none")]
    pub without_headers: Option<::std::collections::HashMap<String, super::StringMatch>>,
    /// Source namespace constraining the applicability of a rule to workloads in that namespace. If the VirtualService has a list of gateways specified in the top-level `gateways` field, it must include the reserved gateway `mesh` for this field to be applicable.
    #[serde(rename = "sourceNamespace", skip_serializing_if = "Option::is_none")]
    pub source_namespace: Option<String>,
}

impl HttpMatchRequest {
    /// HttpMatchRequest specifies a set of criterion to be met in order for the rule to be applied to the HTTP request. For example, the following restricts the rule to match only requests where the URL path starts with /ratings/v2/ and the request contains a custom `end-user` header with value `jason`.
    pub fn new() -> HttpMatchRequest {
        HttpMatchRequest {
            name: None,
            method: None,
            port: None,
            gateways: None,
            headers: None,
            uri: None,
            scheme: None,
            authority: None,
            source_labels: None,
            query_params: None,
            ignore_uri_case: None,
            without_headers: None,
            source_namespace: None,
        }
    }
}