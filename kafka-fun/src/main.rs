extern crate kafka;
extern crate env_logger;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};
use std::str;

/// This program demonstrates consuming messages through a `Consumer`.
/// This is a convenient client that will fit most use cases.  Note
/// that messages must be marked and commited as consumed to ensure
/// only once delivery.
fn main() {
    env_logger::init();

    let broker = "localhost:9092".to_owned();
    let topic = "topicrs";
    let group = "my-group".to_owned();

    let data = "hello, kafka".as_bytes();
    if let Err(e) = produce_message(data, topic, vec![broker.to_owned()]) {
        println!("Failed producing messages: {}", e);
    }

    if let Err(e) = consume_messages(group, topic.to_owned(), vec![broker]) {
        println!("Failed consuming messages: {}", e);
    }
}

fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut con = try!(
        Consumer::from_hosts(brokers)
            .with_topic(topic)
            .with_group(group)
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
    );

    loop {
        let mss = try!(con.poll());
        if mss.is_empty() {
            println!("No messages available right now.");
            return Ok(());
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                println!("{}:{}@{}: {:?}", ms.topic(), ms.partition(),
                                           m.offset, str::from_utf8(m.value).unwrap());
            }
            let _ = con.consume_messageset(ms);
        }
        try!(con.commit_consumed());
    }
}

fn produce_message<'a, 'b>(
    data: &'a [u8],
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    // ~ create a producer. this is a relatively costly operation, so
    // you'll do this typically once in your application and re-use
    // the instance many times.
    let mut producer = try!(
        Producer::from_hosts(brokers)
             // ~ give the brokers one second time to ack the message
             .with_ack_timeout(Duration::from_secs(1))
             // ~ require only one broker to ack the message
             .with_required_acks(RequiredAcks::One)
             // ~ build the producer with the above settings
             .create()
    );

    // ~ now send a single message.  this is a synchronous/blocking
    // operation.

    // ~ we're sending 'data' as a 'value'. there will be no key
    // associated with the sent message.

    // ~ we leave the partition "unspecified" - this is a negative
    // partition - which causes the producer to find out one on its
    // own using its underlying partitioner.
    try!(producer.send(&Record {
        topic: topic,
        partition: -1,
        key: (),
        value: data,
    }));

    // ~ we can achieve exactly the same as above in a shorter way with
    // the following call
    try!(producer.send(&Record::from_value(topic, data)));

    Ok(())
}