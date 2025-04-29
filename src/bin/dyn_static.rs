use flarrow_file_ext::prelude::*;
use flarrow_flows::prelude::*;
use flarrow_layout::prelude::*;
use flarrow_runtime::prelude::{thirdparty::*, *};
use flarrow_url_scheme::prelude::*;
use sink::BenchmarkSink;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting dynamic(source)-static(sink) Rust benchmark");

    let mut layout = DataflowLayout::new();

    let (source, (source_latency, source_throughput)) = layout
        .node("source", async |io: &mut NodeIOBuilder| {
            (io.output("latency"), io.output("throughput"))
        })
        .await;

    let (sink, (sink_latency, sink_throughput)) = layout
        .node("sink", async |io: &mut NodeIOBuilder| {
            (io.input("latency"), io.input("throughput"))
        })
        .await;

    let layout = layout.build();

    let flows = Flows::new(layout.clone(), async move |connector: &mut FlowsBuilder| {
        connector.connect(sink_latency, source_latency, Some(128))?;
        connector.connect(sink_throughput, source_throughput, Some(128))?;

        Ok(())
    })
    .await?;

    let runtime = Runtime::new(
        async |_file_ext: &mut FileExtManagerBuilder, _url_scheme: &mut UrlSchemeManagerBuilder| {
            Ok(())
        },
    )
    .await?;

    let path = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = format!("file://{}/target/release", path);

    runtime
        .run(flows, async move |loader: &mut NodeLoader| {
            let source_file = Url::parse(&format!("{}/libsource.so", path))?;

            #[cfg(not(feature = "raw"))]
            let sink_cfg: serde_yml::Value = serde_yml::from_str(
                r#"
prefix: ""
suffix: "dynamic-static"
"#,
            )?;

            #[cfg(feature = "raw")]
            let sink_cfg: serde_yml::Value = serde_yml::from_str(
                r#"
prefix: "raw"
suffix: "dynamic-static"
"#,
            )?;

            loader.load::<BenchmarkSink>(sink, sink_cfg).await?;

            loader
                .load_url(source_file, source, serde_yml::from_str("")?)
                .await?;

            Ok(())
        })
        .await
}
