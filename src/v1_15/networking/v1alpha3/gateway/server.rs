use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting edge load balancer.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Server : `Server` describes the properties of the proxy on a given load balancer port. For example,



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Server {
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<Box<super::Port>>,
    /// The ip or the Unix domain socket to which the listener should be bound to. Format: `x.x.x.x` or `unix:///path/to/uds` or `unix://@foobar` (Linux abstract namespace). When using Unix domain sockets, the port number should be 0. This can be used to restrict the reachability of this server to be gateway internal only. This is typically used when a gateway needs to communicate to another mesh service e.g. publishing metrics. In such case, the server created with the specified bind will not be available to external gateway clients.
    #[serde(rename = "bind", skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    /// One or more hosts exposed by this gateway. While typically applicable to HTTP services, it can also be used for TCP services using TLS with SNI. A host is specified as a `dnsName` with an optional `namespace/` prefix. The `dnsName` should be specified using FQDN format, optionally including a wildcard character in the left-most component (e.g., `prod/_*.example.com`). Set the `dnsName` to `*` to select all `VirtualService` hosts from the specified namespace (e.g.,`prod/_*`).
    #[serde(rename = "hosts", skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Box<super::ServerTlsSettings>>,
    /// The loopback IP endpoint or Unix domain socket to which traffic should be forwarded to by default. Format should be `127.0.0.1:PORT` or `unix:///path/to/socket` or `unix://@foobar` (Linux abstract namespace). NOT IMPLEMENTED. $hide_from_docs
    #[serde(rename = "defaultEndpoint", skip_serializing_if = "Option::is_none")]
    pub default_endpoint: Option<String>,
    /// An optional name of the server, when set must be unique across all servers. This will be used for variety of purposes like prefixing stats generated with this name etc.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Server {
    /// `Server` describes the properties of the proxy on a given load balancer port. For example,
    pub fn new() -> Server {
        Server {
            port: None,
            bind: None,
            hosts: None,
            tls: None,
            default_endpoint: None,
            name: None,
        }
    }
}