# kubernetes-rust

[![Client Support Level](https://img.shields.io/badge/kubernetes%20client-alpha-green.svg?style=flat&colorA=306CE8)](http://bit.ly/kubernetes-client-support-badge)

Rust client for [Kubernetes](http://kubernetes.io) API.

## Example

List all Pods on `kube-system`:

```rust
extern crate failure;
extern crate k8s_openapi;
extern crate kubernetes_rust;

use k8s_openapi::v1_10::api::core::v1;
use kubernetes_rust::client::APIClient;
use kubernetes_rust::config;

fn main() {
    let kubeconfig = config::load_kube_config().expect("failed to load kubeconfig");
    let kubeclient = APIClient::new(kubeconfig);
    let req = v1::Pod::list_core_v1_namespaced_pod(
        "kube-system",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).expect("failed to define list pod");
    let list_pod = kubeclient
        .request::<v1::PodList>(req)
        .expect("failed to list up pods");
    println!("{:?}", list_pod);
}
```