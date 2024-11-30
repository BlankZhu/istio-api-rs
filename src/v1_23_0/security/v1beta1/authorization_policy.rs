// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_23_0/security/v1beta1/AuthorizationPolicy.yaml --api-version v1beta1
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "security.istio.io", version = "v1beta1", kind = "AuthorizationPolicy", plural = "authorizationpolicies")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct AuthorizationPolicySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<AuthorizationPolicyAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<AuthorizationPolicyProvider>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AuthorizationPolicyRules>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<AuthorizationPolicySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRef")]
    pub target_ref: Option<AuthorizationPolicyTargetRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRefs")]
    pub target_refs: Option<Vec<AuthorizationPolicyTargetRefs>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AuthorizationPolicyAction {
    #[serde(rename = "ALLOW")]
    Allow,
    #[serde(rename = "DENY")]
    Deny,
    #[serde(rename = "AUDIT")]
    Audit,
    #[serde(rename = "CUSTOM")]
    Custom,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicyProvider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicyRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<AuthorizationPolicyRulesFrom>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<AuthorizationPolicyRulesTo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<Vec<AuthorizationPolicyRulesWhen>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicyRulesFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<AuthorizationPolicyRulesFromSource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicyRulesFromSource {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipBlocks")]
    pub ip_blocks: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notIpBlocks")]
    pub not_ip_blocks: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notNamespaces")]
    pub not_namespaces: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notPrincipals")]
    pub not_principals: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notRemoteIpBlocks")]
    pub not_remote_ip_blocks: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notRequestPrincipals")]
    pub not_request_principals: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteIpBlocks")]
    pub remote_ip_blocks: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestPrincipals")]
    pub request_principals: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicyRulesTo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<AuthorizationPolicyRulesToOperation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicyRulesToOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub methods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notHosts")]
    pub not_hosts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notMethods")]
    pub not_methods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notPaths")]
    pub not_paths: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notPorts")]
    pub not_ports: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicyRulesWhen {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notValues")]
    pub not_values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicySelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicyTargetRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorizationPolicyTargetRefs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub kind: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

