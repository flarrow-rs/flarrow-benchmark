use iridis_benchmark::prelude::{
    thirdparty::iridis::prelude::{thirdparty::*, *},
    *,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting static(source)-static(sink) Rust benchmark");

    benchmark(
        async move |source: NodeLayout, sink: NodeLayout, loader: &mut NodeLoader| {
            #[cfg(not(feature = "raw"))]
            let sink_cfg: serde_yml::Value =
                serde_yml::from_str("prefix: \"\"\nsuffix: \"static-static\"\n")?;

            #[cfg(feature = "raw")]
            let sink_cfg: serde_yml::Value =
                serde_yml::from_str("prefix: \"raw\"\nsuffix: \"static-static\"\n")?;

            loader.load::<BenchmarkSink>(sink, sink_cfg).await?;

            loader
                .load::<BenchmarkSource>(source, serde_yml::from_str("")?)
                .await?;

            Ok(())
        },
    )
    .await
}
