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

/// PodSecurityContext : See k8s.io.api.core.v1.PodSecurityContext.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct PodSecurityContext {
    #[serde(rename = "fsGroup", skip_serializing_if = "Option::is_none")]
    pub fs_group: Option<i64>,
    #[serde(rename = "fsGroupChangePolicy", skip_serializing_if = "Option::is_none")]
    pub fs_group_change_policy: Option<String>,
    #[serde(rename = "runAsGroup", skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    #[serde(rename = "runAsNonRoot", skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    #[serde(rename = "runAsUser", skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    #[serde(rename = "seLinuxOptions", skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<Box<super::SeLinuxOptions>>,
    #[serde(rename = "seccompProfile", skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<Box<super::SeccompProfile>>,
    #[serde(rename = "supplementalGroups", skip_serializing_if = "Option::is_none")]
    pub supplemental_groups: Option<Vec<i64>>,
    #[serde(rename = "sysctls", skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<Vec<super::Sysctl>>,
    #[serde(rename = "windowsOptions", skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<Box<super::WindowsSecurityContextOptions>>,
}

impl PodSecurityContext {
    /// See k8s.io.api.core.v1.PodSecurityContext.
    pub fn new() -> PodSecurityContext {
        PodSecurityContext {
            fs_group: None,
            fs_group_change_policy: None,
            run_as_group: None,
            run_as_non_root: None,
            run_as_user: None,
            se_linux_options: None,
            seccomp_profile: None,
            supplemental_groups: None,
            sysctls: None,
            windows_options: None,
        }
    }
}