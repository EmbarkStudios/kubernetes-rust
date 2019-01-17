/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiCoreV1ContainerState : ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiCoreV1ContainerState {
  #[serde(rename = "running")]
  running: Option<::models::IoK8sApiCoreV1ContainerStateRunning>,
  #[serde(rename = "terminated")]
  terminated: Option<::models::IoK8sApiCoreV1ContainerStateTerminated>,
  #[serde(rename = "waiting")]
  waiting: Option<::models::IoK8sApiCoreV1ContainerStateWaiting>
}

impl IoK8sApiCoreV1ContainerState {
  /// ContainerState holds a possible state of container. Only one of its members may be specified. If none of them is specified, the default one is ContainerStateWaiting.
  pub fn new() -> IoK8sApiCoreV1ContainerState {
    IoK8sApiCoreV1ContainerState {
      running: None,
      terminated: None,
      waiting: None
    }
  }

  pub fn set_running(&mut self, running: ::models::IoK8sApiCoreV1ContainerStateRunning) {
    self.running = Some(running);
  }

  pub fn with_running(mut self, running: ::models::IoK8sApiCoreV1ContainerStateRunning) -> IoK8sApiCoreV1ContainerState {
    self.running = Some(running);
    self
  }

  pub fn running(&self) -> Option<&::models::IoK8sApiCoreV1ContainerStateRunning> {
    self.running.as_ref()
  }

  pub fn reset_running(&mut self) {
    self.running = None;
  }

  pub fn set_terminated(&mut self, terminated: ::models::IoK8sApiCoreV1ContainerStateTerminated) {
    self.terminated = Some(terminated);
  }

  pub fn with_terminated(mut self, terminated: ::models::IoK8sApiCoreV1ContainerStateTerminated) -> IoK8sApiCoreV1ContainerState {
    self.terminated = Some(terminated);
    self
  }

  pub fn terminated(&self) -> Option<&::models::IoK8sApiCoreV1ContainerStateTerminated> {
    self.terminated.as_ref()
  }

  pub fn reset_terminated(&mut self) {
    self.terminated = None;
  }

  pub fn set_waiting(&mut self, waiting: ::models::IoK8sApiCoreV1ContainerStateWaiting) {
    self.waiting = Some(waiting);
  }

  pub fn with_waiting(mut self, waiting: ::models::IoK8sApiCoreV1ContainerStateWaiting) -> IoK8sApiCoreV1ContainerState {
    self.waiting = Some(waiting);
    self
  }

  pub fn waiting(&self) -> Option<&::models::IoK8sApiCoreV1ContainerStateWaiting> {
    self.waiting.as_ref()
  }

  pub fn reset_waiting(&mut self) {
    self.waiting = None;
  }

}


