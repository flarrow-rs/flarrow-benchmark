use csv::Writer;
use flarrow_api::prelude::*;

#[cfg(feature = "raw")]
use arrow_array::UInt8Array;

#[cfg(not(feature = "raw"))]
use message::Image;
use message::{BENCH_LEN, SIZES};

use std::{collections::HashMap, time::Duration};
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

    pub prefix: String,
    pub suffix: String,
}

#[node(runtime = "default_runtime")]
impl Node for BenchmarkSink {
    async fn new(
        mut inputs: Inputs,
        _: Outputs,
        configuration: serde_yml::Value,
    ) -> Result<Box<dyn Node>>
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
            prefix: configuration
                .get("prefix")
                .ok_or_eyre("prefix not found")?
                .as_str()
                .ok_or_eyre("prefix is not a string")?
                .to_string(),
            suffix: configuration
                .get("suffix")
                .ok_or_eyre("suffix not found")?
                .as_str()
                .ok_or_eyre("suffix is not a string")?
                .to_string(),
        }) as Box<dyn Node>)
    }

    async fn start(mut self: Box<Self>) -> Result<()> {
        let mut results_latencies: HashMap<usize, u128> = HashMap::new();
        let mut results_throughputs: HashMap<usize, f64> = HashMap::new();

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
                    record_results(
                        &mut results_latencies,
                        &mut results_throughputs,
                        start,
                        current_size,
                        n,
                        latencies,
                        true,
                    );
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

        record_results(
            &mut results_latencies,
            &mut results_throughputs,
            start,
            current_size,
            n,
            latencies,
            true,
        );

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
                    record_results(
                        &mut results_latencies,
                        &mut results_throughputs,
                        start,
                        current_size,
                        n,
                        latencies,
                        false,
                    );
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

        record_results(
            &mut results_latencies,
            &mut results_throughputs,
            start,
            current_size,
            n,
            latencies,
            false,
        );

        let mut wtr =
            Writer::from_path(format!("out/benchmark-{}-{}.csv", self.prefix, self.suffix))?;

        wtr.write_record(&["Size", "Latency", "Throughput"])?;

        let mut sizes: Vec<usize> = results_latencies.keys().copied().collect();
        sizes.sort();

        for size in sizes {
            if let (Some(&latency), Some(&throughput)) =
                (results_latencies.get(&size), results_throughputs.get(&size))
            {
                wtr.write_record(&[
                    size.to_string(),
                    latency.to_string(),
                    throughput.to_string(),
                ])?;
            }
        }

        wtr.flush()?;

        Ok(())
    }
}

fn record_results(
    results_latencies: &mut HashMap<usize, u128>,
    results_throughputs: &mut HashMap<usize, f64>,
    start: Instant,
    current_size: usize,
    n: u32,
    latencies: Vec<Duration>,
    latency: bool,
) {
    let msg = if latency {
        let avg_latency = latencies.iter().sum::<Duration>() / n;
        let avg_micros = avg_latency.as_micros();
        results_latencies.insert(current_size, avg_micros);
        format!("size {current_size}: {avg_micros} µs")
    } else {
        let duration = start.elapsed();
        let msg_per_sec = n as f64 / duration.as_secs_f64();
        results_throughputs.insert(current_size, msg_per_sec);
        format!("size {current_size}: {msg_per_sec:.0} messages per second.")
    };

    println!("{msg}");
}
