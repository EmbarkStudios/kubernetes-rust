/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAdmissionregistrationV1alpha1Initializer : Initializer describes the name and the failure policy of an initializer, and what resources it applies to.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAdmissionregistrationV1alpha1Initializer {
  /// Name is the identifier of the initializer. It will be added to the object that needs to be initialized. Name should be fully qualified, e.g., alwayspullimages.kubernetes.io, where \"alwayspullimages\" is the name of the webhook, and kubernetes.io is the name of the organization. Required
  #[serde(rename = "name")]
  name: String,
  /// Rules describes what resources/subresources the initializer cares about. The initializer cares about an operation if it matches _any_ Rule. Rule.Resources must not include subresources.
  #[serde(rename = "rules")]
  rules: Option<Vec<::models::IoK8sApiAdmissionregistrationV1alpha1Rule>>
}

impl IoK8sApiAdmissionregistrationV1alpha1Initializer {
  /// Initializer describes the name and the failure policy of an initializer, and what resources it applies to.
  pub fn new(name: String) -> IoK8sApiAdmissionregistrationV1alpha1Initializer {
    IoK8sApiAdmissionregistrationV1alpha1Initializer {
      name: name,
      rules: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> IoK8sApiAdmissionregistrationV1alpha1Initializer {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_rules(&mut self, rules: Vec<::models::IoK8sApiAdmissionregistrationV1alpha1Rule>) {
    self.rules = Some(rules);
  }

  pub fn with_rules(mut self, rules: Vec<::models::IoK8sApiAdmissionregistrationV1alpha1Rule>) -> IoK8sApiAdmissionregistrationV1alpha1Initializer {
    self.rules = Some(rules);
    self
  }

  pub fn rules(&self) -> Option<&Vec<::models::IoK8sApiAdmissionregistrationV1alpha1Rule>> {
    self.rules.as_ref()
  }

  pub fn reset_rules(&mut self) {
    self.rules = None;
  }

}


