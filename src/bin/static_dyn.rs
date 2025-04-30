use flarrow_benchmark::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting static(source)-dynamic(sink) Rust benchmark");

    benchmark(
        async move |source: NodeLayout, sink: NodeLayout, loader: &mut NodeLoader| {
            #[cfg(not(feature = "raw"))]
            let sink_cfg: serde_yml::Value =
                serde_yml::from_str("prefix: \"\"\nsuffix: \"static-dynamic\"\n")?;

            #[cfg(feature = "raw")]
            let sink_cfg: serde_yml::Value =
                serde_yml::from_str("prefix: \"raw\"\nsuffix: \"static-dynamic\"\n")?;

            loader.load_url(sink_dylib()?, sink, sink_cfg).await?;

            loader
                .load::<BenchmarkSource>(source, serde_yml::from_str("")?)
                .await?;

            Ok(())
        },
    )
    .await
}
