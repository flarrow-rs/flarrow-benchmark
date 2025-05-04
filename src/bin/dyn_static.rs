use iridis_benchmark::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting dynamic(source)-static(sink) Rust benchmark");

    benchmark(
        async move |source: NodeLayout, sink: NodeLayout, loader: &mut NodeLoader| {
            #[cfg(not(feature = "raw"))]
            let sink_cfg: serde_yml::Value =
                serde_yml::from_str("prefix: \"\"\nsuffix: \"dynamic-static\"\n")?;

            #[cfg(feature = "raw")]
            let sink_cfg: serde_yml::Value =
                serde_yml::from_str("prefix: \"raw\"\nsuffix: \"dynamic-static\"\n")?;

            loader.load::<BenchmarkSink>(sink, sink_cfg).await?;

            loader
                .load_url(source_dylib()?, source, serde_yml::from_str("")?)
                .await?;

            Ok(())
        },
    )
    .await
}
