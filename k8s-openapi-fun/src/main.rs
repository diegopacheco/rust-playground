extern crate k8s_openapi;
use k8s_openapi::api::core::v1 as api;

fn main() {
    let pod_spec: api::PodSpec = Default::default();
    println!("{:#?}", pod_spec);
}
