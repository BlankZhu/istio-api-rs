use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MetricSelectorIstioMetric : Curated list of known metric types that is supported by Istio metric providers. See also: https://istio.io/latest/docs/reference/config/metrics/#metrics

/// Curated list of known metric types that is supported by Istio metric providers. See also: https://istio.io/latest/docs/reference/config/metrics/#metrics
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, JsonSchema)]
pub enum MetricSelectorIstioMetric {
    #[serde(rename = "ALL_METRICS")]
    ALLMETRICS,
    #[serde(rename = "REQUEST_COUNT")]
    REQUESTCOUNT,
    #[serde(rename = "REQUEST_DURATION")]
    REQUESTDURATION,
    #[serde(rename = "REQUEST_SIZE")]
    REQUESTSIZE,
    #[serde(rename = "RESPONSE_SIZE")]
    RESPONSESIZE,
    #[serde(rename = "TCP_OPENED_CONNECTIONS")]
    TCPOPENEDCONNECTIONS,
    #[serde(rename = "TCP_CLOSED_CONNECTIONS")]
    TCPCLOSEDCONNECTIONS,
    #[serde(rename = "TCP_SENT_BYTES")]
    TCPSENTBYTES,
    #[serde(rename = "TCP_RECEIVED_BYTES")]
    TCPRECEIVEDBYTES,
    #[serde(rename = "GRPC_REQUEST_MESSAGES")]
    GRPCREQUESTMESSAGES,
    #[serde(rename = "GRPC_RESPONSE_MESSAGES")]
    GRPCRESPONSEMESSAGES,

}

impl ToString for MetricSelectorIstioMetric {
    fn to_string(&self) -> String {
        match self {
            Self::ALLMETRICS => String::from("ALL_METRICS"),
            Self::REQUESTCOUNT => String::from("REQUEST_COUNT"),
            Self::REQUESTDURATION => String::from("REQUEST_DURATION"),
            Self::REQUESTSIZE => String::from("REQUEST_SIZE"),
            Self::RESPONSESIZE => String::from("RESPONSE_SIZE"),
            Self::TCPOPENEDCONNECTIONS => String::from("TCP_OPENED_CONNECTIONS"),
            Self::TCPCLOSEDCONNECTIONS => String::from("TCP_CLOSED_CONNECTIONS"),
            Self::TCPSENTBYTES => String::from("TCP_SENT_BYTES"),
            Self::TCPRECEIVEDBYTES => String::from("TCP_RECEIVED_BYTES"),
            Self::GRPCREQUESTMESSAGES => String::from("GRPC_REQUEST_MESSAGES"),
            Self::GRPCRESPONSEMESSAGES => String::from("GRPC_RESPONSE_MESSAGES"),
        }
    }
}

impl Default for MetricSelectorIstioMetric {
    fn default() -> MetricSelectorIstioMetric {
        Self::ALLMETRICS
    }
}