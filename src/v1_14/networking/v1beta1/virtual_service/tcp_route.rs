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

/// TcpRoute : Describes match conditions and actions for routing TCP traffic. The following routing rule forwards traffic arriving at port 27017 for mongo.prod.svc.cluster.local to another Mongo server on port 5555.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct TcpRoute {
    /// Match conditions to be satisfied for the rule to be activated. All conditions inside a single match block have AND semantics, while the list of match blocks have OR semantics. The rule is matched if any one of the match blocks succeed.
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<Vec<super::L4MatchAttributes>>,
    /// The destination to which the connection should be forwarded to.
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<super::RouteDestination>>,
}

impl TcpRoute {
    /// Describes match conditions and actions for routing TCP traffic. The following routing rule forwards traffic arriving at port 27017 for mongo.prod.svc.cluster.local to another Mongo server on port 5555.
    pub fn new() -> TcpRoute {
        TcpRoute {
            _match: None,
            route: None,
        }
    }
}