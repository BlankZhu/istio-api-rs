use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
/*
 * Configuration affecting load balancing, outlier detection, etc.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1beta1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// LocalityLoadBalancerSetting : Locality-weighted load balancing allows administrators to control the distribution of traffic to endpoints based on the localities of where the traffic originates and where it will terminate. These localities are specified using arbitrary labels that designate a hierarchy of localities in {region}/{zone}/{sub-zone} form. For additional detail refer to [Locality Weight](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/upstream/load_balancing/locality_weight) The following example shows how to setup locality weights mesh-wide.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct LocalityLoadBalancerSetting {
    /// Optional: only one of distribute, failover or failoverPriority can be set. Explicitly specify loadbalancing weight across different zones and geographical locations. Refer to [Locality weighted load balancing](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/upstream/load_balancing/locality_weight) If empty, the locality weight is set according to the endpoints number within it.
    #[serde(rename = "distribute", skip_serializing_if = "Option::is_none")]
    pub distribute: Option<Vec<super::LocalityLoadBalancerSettingDistribute>>,
    /// Optional: only one of distribute, failover or failoverPriority can be set. Explicitly specify the region traffic will land on when endpoints in local region becomes unhealthy. Should be used together with OutlierDetection to detect unhealthy endpoints. Note: if no OutlierDetection specified, this will not take effect.
    #[serde(rename = "failover", skip_serializing_if = "Option::is_none")]
    pub failover: Option<Vec<super::LocalityLoadBalancerSettingFailover>>,
    /// failoverPriority is an ordered list of labels used to sort endpoints to do priority based load balancing. This is to support traffic failover across different groups of endpoints. Suppose there are total N labels specified: 1. Endpoints matching all N labels with the client proxy have priority P(0) i.e. the highest priority. 2. Endpoints matching the first N-1 labels with the client proxy have priority P(1) i.e. second highest priority. 3. By extension of this logic, endpoints matching only the first label with the client proxy has priority P(N-1) i.e. second lowest priority. 4. All the other endpoints have priority P(N) i.e. lowest priority.
    #[serde(rename = "failoverPriority", skip_serializing_if = "Option::is_none")]
    pub failover_priority: Option<Vec<String>>,
    /// enable locality load balancing, this is DestinationRule-level and will override mesh wide settings in entirety. e.g. true means that turn on locality load balancing for this DestinationRule no matter what mesh wide settings is.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl LocalityLoadBalancerSetting {
    /// Locality-weighted load balancing allows administrators to control the distribution of traffic to endpoints based on the localities of where the traffic originates and where it will terminate. These localities are specified using arbitrary labels that designate a hierarchy of localities in {region}/{zone}/{sub-zone} form. For additional detail refer to [Locality Weight](https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/upstream/load_balancing/locality_weight) The following example shows how to setup locality weights mesh-wide.
    pub fn new() -> LocalityLoadBalancerSetting {
        LocalityLoadBalancerSetting {
            distribute: None,
            failover: None,
            failover_priority: None,
            enabled: None,
        }
    }
}