extern crate rumqtt;

use rumqtt::{MqttClient, MqttOptions, QoS, ReconnectOptions};
use std::{thread, time::Duration};

fn main() {
    pretty_env_logger::init();
 
    //let broker = "localhost";
    //let port = 8883;
    //let client_id = "pubsubClient1";
    let topic_subscribe = "test";
    // Include AWS CA Certificate here (https://www.amazontrust.com/repository/AmazonRootCA1.pem)
    //let ca = include_bytes!("certs/ca-chain.cert.pem").to_vec();
    // Include your IOT Certificate here
    //let client_cert = include_bytes!("certs/cert.pem").to_vec();
    // Include your IOT key here
    //let client_key = include_bytes!("certs/key.pem").to_vec();

    //let reconnection_options = ReconnectOptions::Always(3);
    //let mqtt_options = MqttOptions::new(client_id, broker, port)
       // .set_ca(ca)
       // .set_client_auth(client_cert, client_key)
       // .set_keep_alive(10)
       // .set_reconnect_opts(reconnection_options)
       // .set_clean_session(false);

    let mqtt_options = MqttOptions::new("test-pubsub1", "localhost", 1883);

    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    mqtt_client.subscribe(topic_subscribe, QoS::AtLeastOnce).unwrap();

    thread::spawn(move || {
        for i in 0..10 {
            let payload = format!("message: {}", i);
            thread::sleep(Duration::from_millis(1000));
            mqtt_client.publish(topic_subscribe, QoS::AtLeastOnce, false, payload).unwrap();
        }
    });

    for notification in notifications {
        println!("RECEIVED: {:?}", notification);
    }

    println!("fin.");
}