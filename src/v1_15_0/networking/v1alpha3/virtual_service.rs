// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_15_0/networking/v1alpha3/VirtualService.yaml --api-version v1alpha3
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "VirtualService", plural = "virtualservices")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct VirtualServiceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exportTo")]
    pub export_to: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<Vec<VirtualServiceHttp>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<Vec<VirtualServiceTcp>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<Vec<VirtualServiceTls>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttp {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "corsPolicy")]
    pub cors_policy: Option<VirtualServiceHttpCorsPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegate: Option<VirtualServiceHttpDelegate>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "directResponse")]
    pub direct_response: Option<VirtualServiceHttpDirectResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<VirtualServiceHttpFault>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<VirtualServiceHttpHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<VirtualServiceHttpMatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror: Option<VirtualServiceHttpMirror>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirrorPercent")]
    pub mirror_percent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirrorPercentage")]
    pub mirror_percentage: Option<VirtualServiceHttpMirrorPercentage>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirror_percent")]
    pub mirror_percent_x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<VirtualServiceHttpRedirect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retries: Option<VirtualServiceHttpRetries>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rewrite: Option<VirtualServiceHttpRewrite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<VirtualServiceHttpRoute>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpCorsPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCredentials")]
    pub allow_credentials: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowHeaders")]
    pub allow_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowMethods")]
    pub allow_methods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowOrigin")]
    pub allow_origin: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowOrigins")]
    pub allow_origins: Option<Vec<VirtualServiceHttpCorsPolicyAllowOrigins>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exposeHeaders")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpCorsPolicyAllowOrigins {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpDelegate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpDirectResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<VirtualServiceHttpDirectResponseBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpDirectResponseBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpFault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abort: Option<VirtualServiceHttpFaultAbort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<VirtualServiceHttpFaultDelay>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpFaultAbort {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grpcStatus")]
    pub grpc_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "http2Error")]
    pub http2_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpStatus")]
    pub http_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<VirtualServiceHttpFaultAbortPercentage>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpFaultAbortPercentage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpFaultDelay {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exponentialDelay")]
    pub exponential_delay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fixedDelay")]
    pub fixed_delay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<VirtualServiceHttpFaultDelayPercentage>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpFaultDelayPercentage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<VirtualServiceHttpHeadersRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<VirtualServiceHttpHeadersResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpHeadersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpHeadersResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<VirtualServiceHttpMatchAuthority>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, VirtualServiceHttpMatchHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreUriCase")]
    pub ignore_uri_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<VirtualServiceHttpMatchMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryParams")]
    pub query_params: Option<BTreeMap<String, VirtualServiceHttpMatchQueryParams>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<VirtualServiceHttpMatchScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceNamespace")]
    pub source_namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statPrefix")]
    pub stat_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<VirtualServiceHttpMatchUri>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "withoutHeaders")]
    pub without_headers: Option<BTreeMap<String, VirtualServiceHttpMatchWithoutHeaders>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMatchAuthority {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMatchHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMatchMethod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMatchQueryParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMatchScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMatchUri {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMatchWithoutHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMirror {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<VirtualServiceHttpMirrorPort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMirrorPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpMirrorPercentage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpRedirect {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "derivePort")]
    pub derive_port: Option<VirtualServiceHttpRedirectDerivePort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redirectCode")]
    pub redirect_code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum VirtualServiceHttpRedirectDerivePort {
    #[serde(rename = "FROM_PROTOCOL_DEFAULT")]
    FromProtocolDefault,
    #[serde(rename = "FROM_REQUEST_PORT")]
    FromRequestPort,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpRetries {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "perTryTimeout")]
    pub per_try_timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryOn")]
    pub retry_on: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryRemoteLocalities")]
    pub retry_remote_localities: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpRewrite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpRoute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<VirtualServiceHttpRouteDestination>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<VirtualServiceHttpRouteHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpRouteDestination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<VirtualServiceHttpRouteDestinationPort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpRouteDestinationPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpRouteHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<VirtualServiceHttpRouteHeadersRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<VirtualServiceHttpRouteHeadersResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpRouteHeadersRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceHttpRouteHeadersResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTcp {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<VirtualServiceTcpMatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<VirtualServiceTcpRoute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTcpMatch {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationSubnets")]
    pub destination_subnets: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceNamespace")]
    pub source_namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceSubnet")]
    pub source_subnet: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTcpRoute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<VirtualServiceTcpRouteDestination>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTcpRouteDestination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<VirtualServiceTcpRouteDestinationPort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTcpRouteDestinationPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<VirtualServiceTlsMatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Vec<VirtualServiceTlsRoute>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTlsMatch {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationSubnets")]
    pub destination_subnets: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sniHosts")]
    pub sni_hosts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceLabels")]
    pub source_labels: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceNamespace")]
    pub source_namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTlsRoute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<VirtualServiceTlsRouteDestination>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTlsRouteDestination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<VirtualServiceTlsRouteDestinationPort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VirtualServiceTlsRouteDestinationPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

