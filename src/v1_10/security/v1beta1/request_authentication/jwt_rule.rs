use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Request authentication configuration for workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JwtRule : JSON Web Token (JWT) token format for authentication as defined by [RFC 7519](https://tools.ietf.org/html/rfc7519). See [OAuth 2.0](https://tools.ietf.org/html/rfc6749) and [OIDC 1.0](http://openid.net/connect) for how this is used in the whole authentication flow.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct JwtRule {
    /// Identifies the issuer that issued the JWT. See [issuer](https://tools.ietf.org/html/rfc7519#section-4.1.1) A JWT with different `iss` claim will be rejected.
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// The list of JWT [audiences](https://tools.ietf.org/html/rfc7519#section-4.1.3). that are allowed to access. A JWT containing any of these audiences will be accepted.
    #[serde(rename = "audiences", skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<String>>,
    /// URL of the provider's public key set to validate signature of the JWT. See [OpenID Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata).
    #[serde(rename = "jwksUri", skip_serializing_if = "Option::is_none")]
    pub jwks_uri: Option<String>,
    /// JSON Web Key Set of public keys to validate signature of the JWT. See https://auth0.com/docs/jwks.
    #[serde(rename = "jwks", skip_serializing_if = "Option::is_none")]
    pub jwks: Option<String>,
    /// List of header locations from which JWT is expected. For example, below is the location spec if JWT is expected to be found in `x-jwt-assertion` header, and have \"Bearer \" prefix: ``` fromHeaders: - name: x-jwt-assertion prefix: \"Bearer \" ```
    #[serde(rename = "fromHeaders", skip_serializing_if = "Option::is_none")]
    pub from_headers: Option<Vec<super::JwtHeader>>,
    /// List of query parameters from which JWT is expected. For example, if JWT is provided via query parameter `my_token` (e.g /path?my_token=<JWT>), the config is: ``` fromParams: - \"my_token\" ```
    #[serde(rename = "fromParams", skip_serializing_if = "Option::is_none")]
    pub from_params: Option<Vec<String>>,
    /// This field specifies the header name to output a successfully verified JWT payload to the backend. The forwarded data is `base64_encoded(jwt_payload_in_JSON)`. If it is not specified, the payload will not be emitted.
    #[serde(rename = "outputPayloadToHeader", skip_serializing_if = "Option::is_none")]
    pub output_payload_to_header: Option<String>,
    /// If set to true, the orginal token will be kept for the ustream request. Default is false.
    #[serde(rename = "forwardOriginalToken", skip_serializing_if = "Option::is_none")]
    pub forward_original_token: Option<bool>,
}

impl JwtRule {
    /// JSON Web Token (JWT) token format for authentication as defined by [RFC 7519](https://tools.ietf.org/html/rfc7519). See [OAuth 2.0](https://tools.ietf.org/html/rfc6749) and [OIDC 1.0](http://openid.net/connect) for how this is used in the whole authentication flow.
    pub fn new() -> JwtRule {
        JwtRule {
            issuer: None,
            audiences: None,
            jwks_uri: None,
            jwks: None,
            from_headers: None,
            from_params: None,
            output_payload_to_header: None,
            forward_original_token: None,
        }
    }
}