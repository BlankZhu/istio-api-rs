// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_15_0/security/v1beta1/RequestAuthentication.yaml --api-version v1beta1
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "security.istio.io", version = "v1beta1", kind = "RequestAuthentication", plural = "requestauthentications")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct RequestAuthenticationSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwtRules")]
    pub jwt_rules: Option<Vec<RequestAuthenticationJwtRules>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<RequestAuthenticationSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestAuthenticationJwtRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forwardOriginalToken")]
    pub forward_original_token: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromHeaders")]
    pub from_headers: Option<Vec<RequestAuthenticationJwtRulesFromHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromParams")]
    pub from_params: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwks: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwksUri")]
    pub jwks_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwks_uri")]
    pub jwks_uri_x: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputPayloadToHeader")]
    pub output_payload_to_header: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestAuthenticationJwtRulesFromHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestAuthenticationSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

