use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Telemetry configuration for workloads.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Tracing : Tracing configures tracing behavior for workloads within a mesh. It can be used to enable/disable tracing, as well as to set sampling rates and custom tag extraction.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Tracing {
    /// Optional. Configures additional custom tags to the generated trace spans.
    #[serde(rename = "customTags", skip_serializing_if = "Option::is_none")]
    pub custom_tags: Option<::std::collections::HashMap<String, super::TracingCustomTag>>,
    /// Controls span reporting. If set to true, no spans will be reported for impacted workloads. This does NOT impact context propagation or trace sampling behavior.
    #[serde(rename = "disableSpanReporting", skip_serializing_if = "Option::is_none")]
    pub disable_span_reporting: Option<bool>,
    /// Optional. Name of provider(s) to use for span reporting. If a provider is not specified, the [default tracing provider][istio.mesh.v1alpha1.MeshConfig.default_providers.tracing] will be used. NOTE: At the moment, only a single provider can be specified in a given Tracing rule.
    #[serde(rename = "providers", skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<super::ProviderRef>>,
    /// Controls the rate at which traffic will be selected for tracing if no prior sampling decision has been made. If a prior sampling decision has been made, that decision will be respected. However, if no sampling decision has been made (example: no `x-b3-sampled` tracing header was present in the requests), the traffic will be selected for telemetry generation at the percentage specified.
    #[serde(rename = "randomSamplingPercentage", skip_serializing_if = "Option::is_none")]
    pub random_sampling_percentage: Option<f32>,
    /// This value is true by default; Envoy decides whether or not to sample based on the value of the Request ID generated by Ingress in distributed tracing. The format of this Request ID is specific to Envoy, and if the Request ID generated by the proxy that receives user traffic first is not specific to Envoy, Envoy will break the trace because it cannot interpret the Request ID. By setting this value to false, we can prevent Envoy from sampling based on the Request ID. As a result, the trace will not be broken even if the Request ID is not in the Envoy format. [Trace Context Propagation](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/observability/tracing#trace-context-propagation) provides more information on Request ID handling. $hide_from_docs
    #[serde(rename = "useRequestIdForTraceSampling", skip_serializing_if = "Option::is_none")]
    pub use_request_id_for_trace_sampling: Option<bool>,
}

impl Tracing {
    /// Tracing configures tracing behavior for workloads within a mesh. It can be used to enable/disable tracing, as well as to set sampling rates and custom tag extraction.
    pub fn new() -> Tracing {
        Tracing {
            custom_tags: None,
            disable_span_reporting: None,
            providers: None,
            random_sampling_percentage: None,
            use_request_id_for_trace_sampling: None,
        }
    }
}