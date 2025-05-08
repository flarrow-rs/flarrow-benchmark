use iridis_benchmark::prelude::{
    thirdparty::iridis::prelude::{thirdparty::*, *},
    *,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting dynamic(source)-dynamic(sink) Rust benchmark");

    benchmark(
        async move |source: NodeLayout, sink: NodeLayout, loader: &mut NodeLoader| {
            #[cfg(not(feature = "raw"))]
            let sink_cfg: serde_yml::Value =
                serde_yml::from_str("prefix: \"\"\nsuffix: \"dynamic-dynamic\"\n")?;

            #[cfg(feature = "raw")]
            let sink_cfg: serde_yml::Value =
                serde_yml::from_str("prefix: \"raw\"\nsuffix: \"dynamic-dynamic\"\n")?;

            loader.load_url(sink_dylib()?, sink, sink_cfg).await?;

            loader
                .load_url(source_dylib()?, source, serde_yml::from_str("")?)
                .await?;

            Ok(())
        },
    )
    .await
}
