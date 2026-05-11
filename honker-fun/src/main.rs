use honker::{Database, EnqueueOpts, QueueOpts};
use serde_json::{Value, json};

fn main() {
    let db = Database::open("app.db").expect("open db");
    let q = db.queue("emails", QueueOpts::default());

    let recipients = ["alice@example.com", "bob@example.com", "carol@example.com"];
    for to in recipients {
        let id = q
            .enqueue(
                &json!({ "to": to, "subject": "hello from honker" }),
                EnqueueOpts::default(),
            )
            .expect("enqueue");
        println!("enqueued job id={id} to={to}");
    }

    let worker = "worker-1";
    let mut processed = 0;
    while let Some(job) = q.claim_one(worker).expect("claim") {
        let payload: Value = job.payload_as().expect("payload json");
        println!(
            "processing id={} attempts={} worker={} payload={}",
            job.id, job.attempts, job.worker_id, payload
        );
        let acked = job.ack().expect("ack");
        println!("  ack={acked}");
        processed += 1;
    }

    println!("processed {processed} jobs");
}
