extern crate futures;
extern crate kube;
#[macro_use]
extern crate log;
extern crate env_logger;
use kube::{
    api::{Api, PostParams},
    client::APIClient,
    config,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    std::env::set_var("RUST_LOG", "info,kube=trace");
    let config = config::load_kube_config().await;
    let client = APIClient::new(config.unwrap());
    let pods = Api::v1Pod(client).within("default");
    let nginx = pods.get("pod/external-http-nginx-deployment-85ff79dd56-8sdvh").await.unwrap();
    let containers = nginx.spec.containers;
    println!("{:#?}",containers);
    Ok(())
    */

    std::env::set_var("RUST_LOG", "info,kube=trace");
    env_logger::init();
    let config = config::load_kube_config().await?;
    let client = APIClient::new(config);

    // Manage pods
    let pods = Api::v1Pod(client).within("default");

    info!("Creating Pod instance blog");
    let p = json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": { "name": "blog" },
        "spec": {
            "containers": [{
              "name": "blog",
              "image": "clux/blog:0.1.0"
            }],
        }
    });

    let pp = PostParams::default();
    match pods.create(&pp, serde_json::to_vec(&p)?).await {
        Ok(o) => {
            assert_eq!(p["metadata"]["name"], o.metadata.name);
            info!("Created {}", o.metadata.name);
            // wait for it..
            std::thread::sleep(std::time::Duration::from_millis(5_000));
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => return Err(e.into()),                        // any other case is probably bad
    }

    // Verify we can get it
    info!("Get Pod blog");
    let p1cpy = pods.get("blog").await?;
    println!("Got blog pod with containers: {:?}", p1cpy.spec.containers);
    assert_eq!(p1cpy.spec.containers[0].name, "blog");

    Ok(())
}
