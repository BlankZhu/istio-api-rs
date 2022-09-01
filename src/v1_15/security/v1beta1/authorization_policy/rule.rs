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

/// Rule : Rule matches requests from a list of sources that perform a list of operations subject to a list of conditions. A match occurs when at least one source, one operation and all conditions matches the request. An empty rule is always matched.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Rule {
    /// Optional. from specifies the source of a request.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<super::RuleFrom>>,
    /// Optional. to specifies the operation of a request.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<super::RuleTo>>,
    /// Optional. when specifies a list of additional conditions of a request.
    #[serde(rename = "when", skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<super::Condition>>,
}

impl Rule {
    /// Rule matches requests from a list of sources that perform a list of operations subject to a list of conditions. A match occurs when at least one source, one operation and all conditions matches the request. An empty rule is always matched.
    pub fn new() -> Rule {
        Rule {
            from: None,
            to: None,
            when: None,
        }
    }
}