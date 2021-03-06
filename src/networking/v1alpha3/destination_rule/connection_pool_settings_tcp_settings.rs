use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting load balancing, outlier detection, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConnectionPoolSettingsTcpSettings : Settings common to both HTTP and TCP upstream connections.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ConnectionPoolSettingsTcpSettings {
    /// Maximum number of HTTP1 /TCP connections to a destination host. Default 2^32-1.
    #[serde(rename = "maxConnections", skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// TCP connection timeout. format: 1h/1m/1s/1ms. MUST BE >=1ms. Default is 10s.
    #[serde(rename = "connectTimeout", skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<String>,
    #[serde(rename = "tcpKeepalive", skip_serializing_if = "Option::is_none")]
    pub tcp_keepalive: Option<Box<super::ConnectionPoolSettingsTcpSettingsTcpKeepalive>>,
}

impl ConnectionPoolSettingsTcpSettings {
    /// Settings common to both HTTP and TCP upstream connections.
    pub fn new() -> ConnectionPoolSettingsTcpSettings {
        ConnectionPoolSettingsTcpSettings {
            max_connections: None,
            connect_timeout: None,
            tcp_keepalive: None,
        }
    }
}