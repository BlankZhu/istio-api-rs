// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_16_0/networking/v1alpha3/EnvoyFilter.yaml --api-version v1alpha3
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "EnvoyFilter", plural = "envoyfilters")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct EnvoyFilterSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configPatches")]
    pub config_patches: Option<Vec<EnvoyFilterConfigPatches>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<EnvoyFilterWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatches {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applyTo")]
    pub apply_to: Option<EnvoyFilterConfigPatchesApplyTo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<EnvoyFilterConfigPatchesMatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<EnvoyFilterConfigPatchesPatch>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EnvoyFilterConfigPatchesApplyTo {
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "LISTENER")]
    Listener,
    #[serde(rename = "FILTER_CHAIN")]
    FilterChain,
    #[serde(rename = "NETWORK_FILTER")]
    NetworkFilter,
    #[serde(rename = "HTTP_FILTER")]
    HttpFilter,
    #[serde(rename = "ROUTE_CONFIGURATION")]
    RouteConfiguration,
    #[serde(rename = "VIRTUAL_HOST")]
    VirtualHost,
    #[serde(rename = "HTTP_ROUTE")]
    HttpRoute,
    #[serde(rename = "CLUSTER")]
    Cluster,
    #[serde(rename = "EXTENSION_CONFIG")]
    ExtensionConfig,
    #[serde(rename = "BOOTSTRAP")]
    Bootstrap,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<EnvoyFilterConfigPatchesMatchCluster>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<EnvoyFilterConfigPatchesMatchContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listener: Option<EnvoyFilterConfigPatchesMatchListener>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<EnvoyFilterConfigPatchesMatchProxy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeConfiguration")]
    pub route_configuration: Option<EnvoyFilterConfigPatchesMatchRouteConfiguration>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatchCluster {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EnvoyFilterConfigPatchesMatchContext {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "SIDECAR_INBOUND")]
    SidecarInbound,
    #[serde(rename = "SIDECAR_OUTBOUND")]
    SidecarOutbound,
    #[serde(rename = "GATEWAY")]
    Gateway,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatchListener {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterChain")]
    pub filter_chain: Option<EnvoyFilterConfigPatchesMatchListenerFilterChain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portName")]
    pub port_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatchListenerFilterChain {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applicationProtocols")]
    pub application_protocols: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationPort")]
    pub destination_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<EnvoyFilterConfigPatchesMatchListenerFilterChainFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transportProtocol")]
    pub transport_protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatchListenerFilterChainFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subFilter")]
    pub sub_filter: Option<EnvoyFilterConfigPatchesMatchListenerFilterChainFilterSubFilter>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatchListenerFilterChainFilterSubFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatchProxy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyVersion")]
    pub proxy_version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatchRouteConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portName")]
    pub port_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhost: Option<EnvoyFilterConfigPatchesMatchRouteConfigurationVhost>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatchRouteConfigurationVhost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRoute>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRoute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRouteAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRouteAction {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "ROUTE")]
    Route,
    #[serde(rename = "REDIRECT")]
    Redirect,
    #[serde(rename = "DIRECT_RESPONSE")]
    DirectResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterConfigPatchesPatch {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterClass")]
    pub filter_class: Option<EnvoyFilterConfigPatchesPatchFilterClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<EnvoyFilterConfigPatchesPatchOperation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EnvoyFilterConfigPatchesPatchFilterClass {
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "AUTHN")]
    Authn,
    #[serde(rename = "AUTHZ")]
    Authz,
    #[serde(rename = "STATS")]
    Stats,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EnvoyFilterConfigPatchesPatchOperation {
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "MERGE")]
    Merge,
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "REMOVE")]
    Remove,
    #[serde(rename = "INSERT_BEFORE")]
    InsertBefore,
    #[serde(rename = "INSERT_AFTER")]
    InsertAfter,
    #[serde(rename = "INSERT_FIRST")]
    InsertFirst,
    #[serde(rename = "REPLACE")]
    Replace,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnvoyFilterWorkloadSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

