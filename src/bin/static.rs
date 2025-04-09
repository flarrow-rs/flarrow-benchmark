use std::sync::Arc;

use flarrow_runtime::prelude::*;
use sink::BenchmarkSink;
use source::BenchmarkSource;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting static(source)-static(sink) Rust benchmark");

    let mut layout = DataflowLayout::new();

    let (source, (source_latency, source_throughput)) = layout
        .create_node(async |io: &mut NodeIO| {
            (io.open_output("latency"), io.open_output("throughput"))
        })
        .await;

    let (sink, (sink_latency, sink_throughput)) = layout
        .create_node(async |io: &mut NodeIO| {
            (io.open_input("latency"), io.open_input("throughput"))
        })
        .await;

    let layout = Arc::new(layout);
    let flows = Flows::new(layout.clone(), async move |connector: &mut Connector| {
        connector.connect(sink_latency, source_latency)?;
        connector.connect(sink_throughput, source_throughput)?;

        Ok(())
    })
    .await?;

    let runtime = DataflowRuntime::new(flows, None, async move |loader: &mut Loader| {
        #[cfg(not(feature = "raw"))]
        let sink_cfg: serde_yml::Value = serde_yml::from_str(
            r#"
prefix: ""
suffix: "static-static"
"#,
        )?;

        #[cfg(feature = "raw")]
        let sink_cfg: serde_yml::Value = serde_yml::from_str(
            r#"
prefix: "raw"
suffix: "static-static"
"#,
        )?;

        loader
            .load_statically_linked::<BenchmarkSource>(source, serde_yml::from_str("")?)
            .await
            .wrap_err("Failed to load BenchmarkSource")?;

        loader
            .load_statically_linked::<BenchmarkSink>(sink, sink_cfg)
            .await
            .wrap_err("Failed to load BenchmarkSink")?;

        Ok(())
    })
    .await?;

    runtime.run().await
}
