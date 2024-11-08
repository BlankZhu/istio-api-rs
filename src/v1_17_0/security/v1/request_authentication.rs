// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_17_0/security/v1/RequestAuthentication.yaml --api-version v1
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use schemars::JsonSchema;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// RequestAuthentication defines what request authentication methods are supported by a workload.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "security.istio.io", version = "v1", kind = "RequestAuthentication", plural = "requestauthentications")]
#[kube(namespaced)]
pub struct RequestAuthenticationSpec {
    /// Define the list of JWTs that can be validated at the selected workloads' proxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwtRules")]
    pub jwt_rules: Option<Vec<RequestAuthenticationJwtRules>>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<RequestAuthenticationSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct RequestAuthenticationJwtRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<String>>,
    /// If set to true, the original token will be kept for the upstream request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forwardOriginalToken")]
    pub forward_original_token: Option<bool>,
    /// List of header locations from which JWT is expected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromHeaders")]
    pub from_headers: Option<Vec<RequestAuthenticationJwtRulesFromHeaders>>,
    /// List of query parameters from which JWT is expected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fromParams")]
    pub from_params: Option<Vec<String>>,
    /// Identifies the issuer that issued the JWT.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// JSON Web Key Set of public keys to validate signature of the JWT.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwks: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwksUri")]
    pub jwks_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwks_uri")]
    pub jwks_uri_x: Option<String>,
    /// This field specifies a list of operations to copy the claim to HTTP headers on a successfully verified token.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputClaimToHeaders")]
    pub output_claim_to_headers: Option<Vec<RequestAuthenticationJwtRulesOutputClaimToHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputPayloadToHeader")]
    pub output_payload_to_header: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct RequestAuthenticationJwtRulesFromHeaders {
    /// The HTTP header name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The prefix that should be stripped before decoding the token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct RequestAuthenticationJwtRulesOutputClaimToHeaders {
    /// The name of the claim to be copied from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<String>,
    /// The name of the header to be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
}

/// Optional.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct RequestAuthenticationSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

