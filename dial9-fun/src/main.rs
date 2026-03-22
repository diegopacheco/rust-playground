use dial9_tokio_telemetry::telemetry::{RotatingWriter, TracedRuntime};
use std::time::Duration;
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let writer = RotatingWriter::new(
        "target/traces/trace.bin",
        20 * 1024 * 1024,
        100 * 1024 * 1024,
    )?;

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.worker_threads(4).enable_all();

    let (runtime, guard) = TracedRuntime::build_and_start(builder, writer)?;

    runtime.block_on(async {
        let counter = Arc::new(Mutex::new(0u64));
        let (tx, mut rx) = mpsc::channel::<u64>(16);

        for i in 0..200 {
            let tx = tx.clone();
            let counter = counter.clone();
            tokio::spawn(async move {
                let val = cpu_work(i);
                {
                    let mut c = counter.lock().await;
                    *c += val;
                }
                tokio::time::sleep(Duration::from_millis(i % 13)).await;
                tx.send(val).await.unwrap();
            });
        }
        drop(tx);

        let collector = tokio::spawn(async move {
            let mut total = 0u64;
            let mut count = 0u64;
            while let Some(v) = rx.recv().await {
                total += v;
                count += 1;
                if count % 10 == 0 {
                    tokio::task::yield_now().await;
                }
            }
            (total, count)
        });

        let mut chain_handles = vec![];
        for wave in 0..10 {
            let h = tokio::spawn(async move {
                let mut sum = 0u64;
                for i in 0..50 {
                    let inner = tokio::spawn(async move {
                        cpu_work(wave * 1000 + i)
                    });
                    sum += inner.await.unwrap();
                    tokio::task::yield_now().await;
                }
                sum
            });
            chain_handles.push(h);
        }

        for (i, h) in chain_handles.into_iter().enumerate() {
            let r = h.await.unwrap();
            println!("Wave {} result: {}", i, r);
        }

        let (total, count) = collector.await.unwrap();
        println!("Collected {} values, total: {}", count, total);

        let final_count = *counter.lock().await;
        println!("Counter: {}", final_count);

        tokio::time::sleep(Duration::from_millis(500)).await;
        println!("Traces written to target/traces/");
    });

    drop(guard);

    Ok(())
}

fn cpu_work(seed: u64) -> u64 {
    let mut x = seed;
    for _ in 0..500_000 {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
    }
    x % 1000
}
