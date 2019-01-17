/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiSettingsV1alpha1PodPresetList : PodPresetList is a list of PodPreset objects.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiSettingsV1alpha1PodPresetList {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Items is a list of schema objects.
  #[serde(rename = "items")]
  items: Vec<::models::IoK8sApiSettingsV1alpha1PodPreset>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  #[serde(rename = "metadata")]
  metadata: Option<::models::IoK8sApimachineryPkgApisMetaV1ListMeta>
}

impl IoK8sApiSettingsV1alpha1PodPresetList {
  /// PodPresetList is a list of PodPreset objects.
  pub fn new(items: Vec<::models::IoK8sApiSettingsV1alpha1PodPreset>) -> IoK8sApiSettingsV1alpha1PodPresetList {
    IoK8sApiSettingsV1alpha1PodPresetList {
      api_version: None,
      items: items,
      kind: None,
      metadata: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> IoK8sApiSettingsV1alpha1PodPresetList {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_items(&mut self, items: Vec<::models::IoK8sApiSettingsV1alpha1PodPreset>) {
    self.items = items;
  }

  pub fn with_items(mut self, items: Vec<::models::IoK8sApiSettingsV1alpha1PodPreset>) -> IoK8sApiSettingsV1alpha1PodPresetList {
    self.items = items;
    self
  }

  pub fn items(&self) -> &Vec<::models::IoK8sApiSettingsV1alpha1PodPreset> {
    &self.items
  }


  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> IoK8sApiSettingsV1alpha1PodPresetList {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ListMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::IoK8sApimachineryPkgApisMetaV1ListMeta) -> IoK8sApiSettingsV1alpha1PodPresetList {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1ListMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

}


