use flarrow_api::prelude::*;

use message::Image;
use std::time::Duration;
use tokio::time::Instant;

#[derive(Node)]
pub struct BenchmarkSink {
    pub latency: Input<Image>,
    pub throughput: Input<Image>,
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
        while let Ok((header, image)) = self.latency.recv_async().await {
            let data_len = image.data.len();

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

            if counter >= 10 * 20 {
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
        while let Ok((header, image)) = self.throughput.recv_async().await {
            let data_len = image.data.len();
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

            if counter >= 10 * 20 {
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
