/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiPolicyV1beta1RunAsGroupStrategyOptions : RunAsGroupStrategyOptions defines the strategy type and any options used to create the strategy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiPolicyV1beta1RunAsGroupStrategyOptions {
  /// ranges are the allowed ranges of gids that may be used. If you would like to force a single gid then supply a single range with the same start and end. Required for MustRunAs.
  #[serde(rename = "ranges")]
  ranges: Option<Vec<::models::IoK8sApiPolicyV1beta1IdRange>>,
  /// rule is the strategy that will dictate the allowable RunAsGroup values that may be set.
  #[serde(rename = "rule")]
  rule: String
}

impl IoK8sApiPolicyV1beta1RunAsGroupStrategyOptions {
  /// RunAsGroupStrategyOptions defines the strategy type and any options used to create the strategy.
  pub fn new(rule: String) -> IoK8sApiPolicyV1beta1RunAsGroupStrategyOptions {
    IoK8sApiPolicyV1beta1RunAsGroupStrategyOptions {
      ranges: None,
      rule: rule
    }
  }

  pub fn set_ranges(&mut self, ranges: Vec<::models::IoK8sApiPolicyV1beta1IdRange>) {
    self.ranges = Some(ranges);
  }

  pub fn with_ranges(mut self, ranges: Vec<::models::IoK8sApiPolicyV1beta1IdRange>) -> IoK8sApiPolicyV1beta1RunAsGroupStrategyOptions {
    self.ranges = Some(ranges);
    self
  }

  pub fn ranges(&self) -> Option<&Vec<::models::IoK8sApiPolicyV1beta1IdRange>> {
    self.ranges.as_ref()
  }

  pub fn reset_ranges(&mut self) {
    self.ranges = None;
  }

  pub fn set_rule(&mut self, rule: String) {
    self.rule = rule;
  }

  pub fn with_rule(mut self, rule: String) -> IoK8sApiPolicyV1beta1RunAsGroupStrategyOptions {
    self.rule = rule;
    self
  }

  pub fn rule(&self) -> &String {
    &self.rule
  }


}


