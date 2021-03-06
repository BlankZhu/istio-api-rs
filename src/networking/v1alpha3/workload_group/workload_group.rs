use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use kube::CustomResource;
/*
 * Describes a collection of workload instances.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha3
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WorkloadGroupSpec : `WorkloadGroup` enables specifying the properties of a single workload for bootstrap and provides a template for `WorkloadEntry`, similar to how `Deployment` specifies properties of workloads via `Pod` templates. A `WorkloadGroup` can have more than one `WorkloadEntry`. `WorkloadGroup` has no relationship to resources which control service registry like `ServiceEntry` and as such doesn't configure host name for these workloads.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema, CustomResource)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "WorkloadGroup", namespaced)]
pub struct WorkloadGroupSpec {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<super::WorkloadGroupObjectMeta>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<super::WorkloadEntry>>,
    #[serde(rename = "probe", skip_serializing_if = "Option::is_none")]
    pub probe: Option<Box<super::ReadinessProbe>>,
}

impl WorkloadGroupSpec {
    /// `WorkloadGroup` enables specifying the properties of a single workload for bootstrap and provides a template for `WorkloadEntry`, similar to how `Deployment` specifies properties of workloads via `Pod` templates. A `WorkloadGroup` can have more than one `WorkloadEntry`. `WorkloadGroup` has no relationship to resources which control service registry like `ServiceEntry` and as such doesn't configure host name for these workloads.
    pub fn new() -> WorkloadGroupSpec {
        WorkloadGroupSpec {
            metadata: None,
            template: None,
            probe: None,
        }
    }
}