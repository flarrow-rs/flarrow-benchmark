use std::{collections::HashMap, time::Duration};

use iridis_api::prelude::{
    thirdparty::{tokio::io::AsyncWriteExt, *},
    *,
};

#[cfg(feature = "raw")]
use iridis_api::prelude::thirdparty::arrow_array::UInt8Array;

#[cfg(not(feature = "raw"))]
use message::Image;
use message::{BENCH_LEN, SIZES};

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
        _: Queries,
        _: Queryables,
        configuration: serde_yml::Value,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
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
        })
    }

    async fn start(mut self: Box<Self>) -> Result<()> {
        let mut latencies_map = HashMap::new();

        for size in SIZES {
            let mut latencies = Vec::new();

            for _ in 0..BENCH_LEN {
                let (header, _) = self.latency.recv().await?;
                let latency = header
                    .timestamp
                    .get_time()
                    .to_system_time()
                    .elapsed()
                    .unwrap_or_default();

                latencies.push(latency);
            }

            let avg_latency = latencies.iter().sum::<Duration>() / latencies.len() as u32;
            latencies_map.insert(size, avg_latency);
        }

        let mut throughputs_map = HashMap::new();

        for size in SIZES {
            let mut throughputs = Vec::new();

            for _ in 0..BENCH_LEN {
                let (header, _) = self.throughput.recv().await?;

                throughputs.push(header.timestamp);
            }

            let avg_duration = if throughputs.len() > 1 {
                let intervals: Vec<Duration> = throughputs
                    .windows(2)
                    .map(|pair| pair[1].get_diff_duration(&pair[0]))
                    .collect();

                let total_duration: Duration = intervals.iter().sum();
                total_duration / (intervals.len() as u32)
            } else {
                Duration::ZERO
            };

            let avg_throughput = match avg_duration {
                Duration::ZERO => None,
                _ => Some(1.0 / avg_duration.as_secs_f32()),
            };

            throughputs_map.insert(size, avg_throughput);
        }

        println!(
            "{:<15} {:>15} {:>15} {:>15}",
            "Latency (µs)", "Throughput (msg/s)", "Throughput (GB/s)", "Size (bytes)",
        );

        for size in SIZES {
            let avg_latency = latencies_map.get(&size).unwrap_or(&Duration::ZERO);
            let throughput = throughputs_map
                .get(&size)
                .unwrap_or(&Some(0f32))
                .unwrap_or(0f32);
            let throughput_gbps = throughput * (size as f32) / 1_000_000_000.0;

            println!(
                "{:<15.3} {:>15.3} {:>15.6} {:>15}",
                avg_latency.as_micros(),
                throughput,
                throughput_gbps,
                size,
            );
        }

        let filename = if self.prefix.is_empty() {
            format!("{}", self.suffix)
        } else {
            format!("{}-{}", self.prefix, self.suffix)
        };

        let mut file = tokio::fs::File::create(format!("out/{}.csv", filename)).await?;

        file.write_all(b"latency_us,throughput_ops,throughput_gbps,size\n")
            .await?;

        // Écrire les données pour chaque taille
        for size in SIZES {
            let avg_latency = latencies_map.get(&size).unwrap_or(&Duration::ZERO);
            let throughput = throughputs_map
                .get(&size)
                .unwrap_or(&Some(0f32))
                .unwrap_or(0f32);
            let throughput_gbps = throughput * (size as f32) / 1_000_000_000.0;

            let line = format!(
                "{:.3},{:.3},{:.6},{}\n",
                avg_latency.as_micros(),
                throughput,
                throughput_gbps,
                size,
            );

            file.write_all(line.as_bytes()).await?;
        }

        Ok(())
    }
}
