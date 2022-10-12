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

/// K8sIoApiCoreV1DownwardApiVolumeSource : DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct K8sIoApiCoreV1DownwardApiVolumeSource {
    /// Optional: mode bits to use on created files by default. Must be a value between 0 and 0777. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set. +optional
    #[serde(rename = "defaultMode", skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// Items is a list of downward API volume file +optional
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<super::K8sIoApiCoreV1DownwardApiVolumeFile>>,
}

impl K8sIoApiCoreV1DownwardApiVolumeSource {
    /// DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.
    pub fn new() -> K8sIoApiCoreV1DownwardApiVolumeSource {
        K8sIoApiCoreV1DownwardApiVolumeSource {
            default_mode: None,
            items: None,
        }
    }
}