// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f resources/istio/v1_16_0/networking/v1alpha3/Sidecar.yaml --api-version v1alpha3
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "Sidecar", plural = "sidecars")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct SidecarSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<SidecarEgress>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<SidecarIngress>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outboundTrafficPolicy")]
    pub outbound_traffic_policy: Option<SidecarOutboundTrafficPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<SidecarWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarEgress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "captureMode")]
    pub capture_mode: Option<SidecarEgressCaptureMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarEgressPort>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SidecarEgressCaptureMode {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "IPTABLES")]
    Iptables,
    #[serde(rename = "NONE")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarEgressPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarIngress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "captureMode")]
    pub capture_mode: Option<SidecarIngressCaptureMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultEndpoint")]
    pub default_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarIngressPort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<SidecarIngressTls>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SidecarIngressCaptureMode {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "IPTABLES")]
    Iptables,
    #[serde(rename = "NONE")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarIngressPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarIngressTls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificates")]
    pub ca_certificates: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cipherSuites")]
    pub cipher_suites: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialName")]
    pub credential_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsRedirect")]
    pub https_redirect: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxProtocolVersion")]
    pub max_protocol_version: Option<SidecarIngressTlsMaxProtocolVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minProtocolVersion")]
    pub min_protocol_version: Option<SidecarIngressTlsMinProtocolVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<SidecarIngressTlsMode>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKey")]
    pub private_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverCertificate")]
    pub server_certificate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateHash")]
    pub verify_certificate_hash: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateSpki")]
    pub verify_certificate_spki: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SidecarIngressTlsMaxProtocolVersion {
    #[serde(rename = "TLS_AUTO")]
    TlsAuto,
    #[serde(rename = "TLSV1_0")]
    Tlsv10,
    #[serde(rename = "TLSV1_1")]
    Tlsv11,
    #[serde(rename = "TLSV1_2")]
    Tlsv12,
    #[serde(rename = "TLSV1_3")]
    Tlsv13,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SidecarIngressTlsMinProtocolVersion {
    #[serde(rename = "TLS_AUTO")]
    TlsAuto,
    #[serde(rename = "TLSV1_0")]
    Tlsv10,
    #[serde(rename = "TLSV1_1")]
    Tlsv11,
    #[serde(rename = "TLSV1_2")]
    Tlsv12,
    #[serde(rename = "TLSV1_3")]
    Tlsv13,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SidecarIngressTlsMode {
    #[serde(rename = "PASSTHROUGH")]
    Passthrough,
    #[serde(rename = "SIMPLE")]
    Simple,
    #[serde(rename = "MUTUAL")]
    Mutual,
    #[serde(rename = "AUTO_PASSTHROUGH")]
    AutoPassthrough,
    #[serde(rename = "ISTIO_MUTUAL")]
    IstioMutual,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarOutboundTrafficPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "egressProxy")]
    pub egress_proxy: Option<SidecarOutboundTrafficPolicyEgressProxy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<SidecarOutboundTrafficPolicyMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarOutboundTrafficPolicyEgressProxy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarOutboundTrafficPolicyEgressProxyPort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarOutboundTrafficPolicyEgressProxyPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SidecarOutboundTrafficPolicyMode {
    #[serde(rename = "REGISTRY_ONLY")]
    RegistryOnly,
    #[serde(rename = "ALLOW_ANY")]
    AllowAny,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarWorkloadSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

