use std::{collections::HashMap, time::Duration};

use flarrow_api::prelude::{
    thirdparty::{arrow_array::UInt8Array, *},
    *,
};

#[cfg(not(feature = "raw"))]
use message::Image;
use message::{BENCH_LEN, SIZES};

use rand::{Rng, distr::StandardUniform};

#[derive(Node)]
pub struct BenchmarkSource {
    #[cfg(not(feature = "raw"))]
    pub latency: Output<Image>,
    #[cfg(feature = "raw")]
    pub latency: Output<UInt8Array>,

    #[cfg(not(feature = "raw"))]
    pub throughput: Output<Image>,
    #[cfg(feature = "raw")]
    pub throughput: Output<UInt8Array>,

    pub data: HashMap<usize, Vec<u8>>,
}

#[node(runtime = "default_runtime")]
impl Node for BenchmarkSource {
    async fn new(
        _: Inputs,
        mut outputs: Outputs,
        _: Queries,
        _: Queryables,
        _: serde_yml::Value,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let mut data = HashMap::new();
        for size in SIZES {
            let vec: Vec<u8> = rand::rng()
                .sample_iter(StandardUniform)
                .take(size)
                .collect();

            data.insert(size, vec);
        }

        Ok(Self {
            latency: outputs
                .with("latency")
                .await
                .wrap_err("Failed to create latency output")?,
            throughput: outputs
                .with("throughput")
                .await
                .wrap_err("Failed to create throughput output")?,
            data,
        })
    }

    async fn start(self: Box<Self>) -> Result<()> {
        // test latency first
        for size in SIZES {
            for _ in 0..BENCH_LEN {
                let data = self
                    .data
                    .get(&size)
                    .ok_or_eyre(format!("Could not get a `data` according to {}", size))?
                    .clone();

                #[cfg(feature = "raw")]
                {
                    self.latency.send_async(UInt8Array::from(data)).await?;
                }
                #[cfg(not(feature = "raw"))]
                {
                    let image = Image {
                        metadata: None,
                        data: UInt8Array::from(data),
                    };

                    self.latency.send_async(image).await?;
                }

                // sleep a bit to avoid queue buildup
                tokio::time::sleep(Duration::from_millis(3)).await;
            }
        }

        // wait a bit to ensure that all throughput messages reached their target
        tokio::time::sleep(Duration::from_secs(1)).await;

        for size in SIZES {
            for _ in 0..BENCH_LEN {
                let data = self
                    .data
                    .get(&size)
                    .ok_or_eyre(format!("Could not get a `data` according to {}", size))?
                    .clone();

                #[cfg(feature = "raw")]
                {
                    self.throughput.send_async(UInt8Array::from(data)).await?;
                }
                #[cfg(not(feature = "raw"))]
                {
                    let image = Image {
                        metadata: None,
                        data: UInt8Array::from(data),
                    };

                    self.throughput.send_async(image).await?;
                }
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        Ok(())
    }
}
