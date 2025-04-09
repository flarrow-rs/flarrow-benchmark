use flarrow_api::prelude::*;

#[cfg(feature = "raw")]
use arrow_array::UInt8Array;

#[cfg(not(feature = "raw"))]
use message::Image;
use message::{BENCH_LEN, SIZES};

use std::time::Duration;
use tokio::time::Instant;

#[derive(Node)]
pub struct BenchmarkSink {
    #[cfg(not(feature = "raw"))]
    pub latency: Input<Image>,
    #[cfg(feature = "raw")]
    pub latency: Input<UInt8Array>,

    #[cfg(not(feature = "raw"))]
    pub throughput: Input<Image>,
    #[cfg(feature = "raw")]
    pub throughput: Input<UInt8Array>,
}

#[node(runtime = "default_runtime")]
impl Node for BenchmarkSink {
    async fn new(mut inputs: Inputs, _: Outputs, _: serde_yml::Value) -> Result<Box<dyn Node>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self {
            latency: inputs
                .with("latency")
                .await
                .wrap_err("Failed to create latency input")?,
            throughput: inputs
                .with("throughput")
                .await
                .wrap_err("Failed to create throughput input")?,
        }) as Box<dyn Node>)
    }

    async fn start(mut self: Box<Self>) -> Result<()> {
        let mut current_size = 0;
        let mut n = 0;
        let mut start = Instant::now();
        let mut latencies = Vec::new();

        let mut counter = 0;

        println!("Latency: ");
        while let Ok((header, data)) = self.latency.recv_async().await {
            #[cfg(feature = "raw")]
            let data_len = data.len();
            #[cfg(not(feature = "raw"))]
            let data_len = data.data.len();

            if data_len != current_size {
                if n > 0 {
                    record_results(start, current_size, n, latencies, true);
                }
                current_size = data_len;
                n = 0;
                start = Instant::now();
                latencies = Vec::new();
            }

            n += 1;
            latencies.push(
                header
                    .timestamp
                    .get_time()
                    .to_system_time()
                    .elapsed()
                    .unwrap_or_default(),
            );

            counter += 1;

            if counter >= SIZES.len() * BENCH_LEN {
                break;
            }
        }

        record_results(start, current_size, n, latencies, true);

        counter = 0;
        let mut current_size = 0;
        let mut n = 0;
        let mut start = Instant::now();
        let mut latencies = Vec::new();

        println!("Throughput: ");
        while let Ok((header, data)) = self.throughput.recv_async().await {
            #[cfg(feature = "raw")]
            let data_len = data.len();
            #[cfg(not(feature = "raw"))]
            let data_len = data.data.len();

            if data_len != current_size {
                if n > 0 {
                    record_results(start, current_size, n, latencies, false);
                }
                current_size = data_len;
                n = 0;
                start = Instant::now();
                latencies = Vec::new();
            }

            n += 1;
            latencies.push(
                header
                    .timestamp
                    .get_time()
                    .to_system_time()
                    .elapsed()
                    .unwrap_or_default(),
            );

            counter += 1;

            if counter >= SIZES.len() * BENCH_LEN {
                break;
            }
        }

        record_results(start, current_size, n, latencies, false);

        Ok(())
    }
}

fn record_results(
    start: Instant,
    current_size: usize,
    n: u32,
    latencies: Vec<Duration>,
    latency: bool,
) {
    let msg = if latency {
        let avg_latency = latencies.iter().sum::<Duration>() / n;
        format!("size {current_size}: {avg_latency:?}")
    } else {
        let duration = start.elapsed();
        let msg_per_sec = n as f64 / duration.as_secs_f64();
        format!("size {current_size}: {msg_per_sec:.0} messages per second.")
    };

    println!("{msg}");
}
