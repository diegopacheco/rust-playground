extern crate nats;
use nats::*;

fn main() {

    let mut client = Client::new("nats://127.0.0.1").unwrap();
    client.set_synchronous(true);
    client.set_name("app");
    client.publish("subject.test", "test".as_bytes()).unwrap();
    let inbox = client.make_request("subject.rpc", "test".as_bytes()).unwrap();
    println!("Receive message from nats: {:?}",inbox);

}