/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiExtensionsV1beta1DaemonSetSpec : DaemonSetSpec is the specification of a daemon set.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiExtensionsV1beta1DaemonSetSpec {
  /// The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready).
  #[serde(rename = "minReadySeconds")]
  min_ready_seconds: Option<i32>,
  /// The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10.
  #[serde(rename = "revisionHistoryLimit")]
  revision_history_limit: Option<i32>,
  #[serde(rename = "selector")]
  selector: Option<::models::IoK8sApimachineryPkgApisMetaV1LabelSelector>,
  #[serde(rename = "template")]
  template: ::models::IoK8sApiCoreV1PodTemplateSpec,
  /// DEPRECATED. A sequence number representing a specific generation of the template. Populated by the system. It can be set only during the creation.
  #[serde(rename = "templateGeneration")]
  template_generation: Option<i64>,
  #[serde(rename = "updateStrategy")]
  update_strategy: Option<::models::IoK8sApiExtensionsV1beta1DaemonSetUpdateStrategy>
}

impl IoK8sApiExtensionsV1beta1DaemonSetSpec {
  /// DaemonSetSpec is the specification of a daemon set.
  pub fn new(template: ::models::IoK8sApiCoreV1PodTemplateSpec) -> IoK8sApiExtensionsV1beta1DaemonSetSpec {
    IoK8sApiExtensionsV1beta1DaemonSetSpec {
      min_ready_seconds: None,
      revision_history_limit: None,
      selector: None,
      template: template,
      template_generation: None,
      update_strategy: None
    }
  }

  pub fn set_min_ready_seconds(&mut self, min_ready_seconds: i32) {
    self.min_ready_seconds = Some(min_ready_seconds);
  }

  pub fn with_min_ready_seconds(mut self, min_ready_seconds: i32) -> IoK8sApiExtensionsV1beta1DaemonSetSpec {
    self.min_ready_seconds = Some(min_ready_seconds);
    self
  }

  pub fn min_ready_seconds(&self) -> Option<&i32> {
    self.min_ready_seconds.as_ref()
  }

  pub fn reset_min_ready_seconds(&mut self) {
    self.min_ready_seconds = None;
  }

  pub fn set_revision_history_limit(&mut self, revision_history_limit: i32) {
    self.revision_history_limit = Some(revision_history_limit);
  }

  pub fn with_revision_history_limit(mut self, revision_history_limit: i32) -> IoK8sApiExtensionsV1beta1DaemonSetSpec {
    self.revision_history_limit = Some(revision_history_limit);
    self
  }

  pub fn revision_history_limit(&self) -> Option<&i32> {
    self.revision_history_limit.as_ref()
  }

  pub fn reset_revision_history_limit(&mut self) {
    self.revision_history_limit = None;
  }

  pub fn set_selector(&mut self, selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: ::models::IoK8sApimachineryPkgApisMetaV1LabelSelector) -> IoK8sApiExtensionsV1beta1DaemonSetSpec {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&::models::IoK8sApimachineryPkgApisMetaV1LabelSelector> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

  pub fn set_template(&mut self, template: ::models::IoK8sApiCoreV1PodTemplateSpec) {
    self.template = template;
  }

  pub fn with_template(mut self, template: ::models::IoK8sApiCoreV1PodTemplateSpec) -> IoK8sApiExtensionsV1beta1DaemonSetSpec {
    self.template = template;
    self
  }

  pub fn template(&self) -> &::models::IoK8sApiCoreV1PodTemplateSpec {
    &self.template
  }


  pub fn set_template_generation(&mut self, template_generation: i64) {
    self.template_generation = Some(template_generation);
  }

  pub fn with_template_generation(mut self, template_generation: i64) -> IoK8sApiExtensionsV1beta1DaemonSetSpec {
    self.template_generation = Some(template_generation);
    self
  }

  pub fn template_generation(&self) -> Option<&i64> {
    self.template_generation.as_ref()
  }

  pub fn reset_template_generation(&mut self) {
    self.template_generation = None;
  }

  pub fn set_update_strategy(&mut self, update_strategy: ::models::IoK8sApiExtensionsV1beta1DaemonSetUpdateStrategy) {
    self.update_strategy = Some(update_strategy);
  }

  pub fn with_update_strategy(mut self, update_strategy: ::models::IoK8sApiExtensionsV1beta1DaemonSetUpdateStrategy) -> IoK8sApiExtensionsV1beta1DaemonSetSpec {
    self.update_strategy = Some(update_strategy);
    self
  }

  pub fn update_strategy(&self) -> Option<&::models::IoK8sApiExtensionsV1beta1DaemonSetUpdateStrategy> {
    self.update_strategy.as_ref()
  }

  pub fn reset_update_strategy(&mut self) {
    self.update_strategy = None;
  }

}


