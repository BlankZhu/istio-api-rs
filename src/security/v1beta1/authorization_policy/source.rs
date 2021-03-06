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

/// Source : Source specifies the source identities of a request. Fields in the source are ANDed together.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Source {
    /// Optional. A list of peer identities derived from the peer certificate. The peer identity is in the format of `\"<TRUST_DOMAIN>/ns/<NAMESPACE>/sa/<SERVICE_ACCOUNT>\"`, for example, `\"cluster.local/ns/default/sa/productpage\"`. This field requires mTLS enabled and is the same as the `source.principal` attribute.
    #[serde(rename = "principals", skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    /// Optional. A list of negative match of peer identities.
    #[serde(rename = "notPrincipals", skip_serializing_if = "Option::is_none")]
    pub not_principals: Option<Vec<String>>,
    /// Optional. A list of request identities derived from the JWT. The request identity is in the format of `\"<ISS>/<SUB>\"`, for example, `\"example.com/sub-1\"`. This field requires request authentication enabled and is the same as the `request.auth.principal` attribute.
    #[serde(rename = "requestPrincipals", skip_serializing_if = "Option::is_none")]
    pub request_principals: Option<Vec<String>>,
    /// Optional. A list of negative match of request identities.
    #[serde(rename = "notRequestPrincipals", skip_serializing_if = "Option::is_none")]
    pub not_request_principals: Option<Vec<String>>,
    /// Optional. A list of namespaces derived from the peer certificate. This field requires mTLS enabled and is the same as the `source.namespace` attribute.
    #[serde(rename = "namespaces", skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// Optional. A list of negative match of namespaces.
    #[serde(rename = "notNamespaces", skip_serializing_if = "Option::is_none")]
    pub not_namespaces: Option<Vec<String>>,
    /// Optional. A list of IP blocks, populated from the source address of the IP packet. Single IP (e.g. \"1.2.3.4\") and CIDR (e.g. \"1.2.3.0/24\") are supported. This is the same as the `source.ip` attribute.
    #[serde(rename = "ipBlocks", skip_serializing_if = "Option::is_none")]
    pub ip_blocks: Option<Vec<String>>,
    /// Optional. A list of negative match of IP blocks.
    #[serde(rename = "notIpBlocks", skip_serializing_if = "Option::is_none")]
    pub not_ip_blocks: Option<Vec<String>>,
    /// Optional. A list of IP blocks, populated from X-Forwarded-For header or proxy protocol. To make use of this field, you must configure the numTrustedProxies field of the gatewayTopology under the meshConfig when you install Istio or using an annotation on the ingress gateway. See the documentation here: [Configuring Gateway Network Topology](https://istio.io/latest/docs/ops/configuration/traffic-management/network-topologies/). Single IP (e.g. \"1.2.3.4\") and CIDR (e.g. \"1.2.3.0/24\") are supported. This is the same as the `remote.ip` attribute.
    #[serde(rename = "remoteIpBlocks", skip_serializing_if = "Option::is_none")]
    pub remote_ip_blocks: Option<Vec<String>>,
    /// Optional. A list of negative match of remote IP blocks.
    #[serde(rename = "notRemoteIpBlocks", skip_serializing_if = "Option::is_none")]
    pub not_remote_ip_blocks: Option<Vec<String>>,
}

impl Source {
    /// Source specifies the source identities of a request. Fields in the source are ANDed together.
    pub fn new() -> Source {
        Source {
            principals: None,
            not_principals: None,
            request_principals: None,
            not_request_principals: None,
            namespaces: None,
            not_namespaces: None,
            ip_blocks: None,
            not_ip_blocks: None,
            remote_ip_blocks: None,
            not_remote_ip_blocks: None,
        }
    }
}