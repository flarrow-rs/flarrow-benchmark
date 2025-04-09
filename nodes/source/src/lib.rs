use std::{collections::HashMap, time::Duration};

use arrow_array::UInt8Array;
use flarrow_api::prelude::*;

use message::Image;
use rand::{Rng, distr::StandardUniform};

const SIZES: [usize; 10] = [
    1,
    8,
    64,
    512,
    2048,
    4096,
    4 * 4096,
    10 * 4096,
    100 * 4096,
    1000 * 4096,
];

#[derive(Node)]
pub struct BenchmarkSource {
    pub latency: Output<Image>,
    pub throughput: Output<Image>,
}

#[node(runtime = "default_runtime")]
impl Node for BenchmarkSource {
    async fn new(_: Inputs, mut outputs: Outputs, _: serde_yml::Value) -> Result<Box<dyn Node>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self {
            latency: outputs
                .with("latency")
                .await
                .wrap_err("Failed to create latency output")?,
            throughput: outputs
                .with("throughput")
                .await
                .wrap_err("Failed to create throughput output")?,
        }) as Box<dyn Node>)
    }

    async fn start(self: Box<Self>) -> Result<()> {
        let mut data = HashMap::new();
        for size in SIZES {
            let vec: Vec<u8> = rand::rng()
                .sample_iter(StandardUniform)
                .take(size)
                .collect();

            data.insert(size, vec);
        }

        // test latency first
        for size in SIZES {
            for _ in 0..20 {
                let data = data
                    .get(&size)
                    .ok_or_eyre(format!("Could not get a `data` according to {}", size))?
                    .clone();

                let image = Image {
                    metadata: None,
                    data: UInt8Array::from(data),
                };

                self.latency.send(image)?;

                // sleep a bit to avoid queue buildup
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }

        // wait a bit to ensure that all throughput messages reached their target
        tokio::time::sleep(Duration::from_secs(3)).await;

        for size in SIZES {
            for _ in 0..20 {
                let data = data
                    .get(&size)
                    .ok_or_eyre(format!("Could not get a `data` according to {}", size))?
                    .clone();

                let image = Image {
                    metadata: None,
                    data: UInt8Array::from(data),
                };

                self.throughput.send(image)?;
            }
        }

        Ok(())
    }
}
