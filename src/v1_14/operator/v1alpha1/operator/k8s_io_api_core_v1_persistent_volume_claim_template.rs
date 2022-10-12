use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/*
 * Configuration affecting Istio control plane installation version and shape.
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1alpha1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// K8sIoApiCoreV1PersistentVolumeClaimTemplate : PersistentVolumeClaimTemplate is used to produce PersistentVolumeClaim objects as part of an EphemeralVolumeSource.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1PersistentVolumeClaimTemplate {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<super::K8sIoApimachineryPkgApisMetaV1ObjectMeta>>,
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<super::K8sIoApiCoreV1PersistentVolumeClaimSpec>>,
}

impl K8sIoApiCoreV1PersistentVolumeClaimTemplate {
    /// PersistentVolumeClaimTemplate is used to produce PersistentVolumeClaim objects as part of an EphemeralVolumeSource.
    pub fn new() -> K8sIoApiCoreV1PersistentVolumeClaimTemplate {
        K8sIoApiCoreV1PersistentVolumeClaimTemplate {
            metadata: None,
            spec: None,
        }
    }
}