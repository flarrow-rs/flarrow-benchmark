pub mod prelude {
    pub use sink::BenchmarkSink;
    pub use source::BenchmarkSource;

    pub use crate::dylib::*;

    pub mod thirdparty {
        pub use iridis;
    }
}

pub(crate) mod dylib {
    use crate::prelude::thirdparty::iridis::prelude::{thirdparty::*, *};

    pub fn dylib(name: impl Into<String>) -> Result<Url> {
        let path = std::env::var("CARGO_MANIFEST_DIR")?;
        let path = format!("file://{}/target/release", path);

        let prefix = std::env::consts::DLL_PREFIX;
        let dylib = std::env::consts::DLL_SUFFIX;

        Url::parse(&format!("{}/{}{}{}", path, prefix, name.into(), dylib))
            .map_err(eyre::Report::msg)
    }

    pub fn source_dylib() -> Result<Url> {
        dylib("source")
    }

    pub fn sink_dylib() -> Result<Url> {
        dylib("sink")
    }

    pub async fn benchmark(
        nodes: impl AsyncFnOnce(NodeLayout, NodeLayout, &mut NodeLoader) -> Result<()>,
    ) -> Result<()> {
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
            async |_file_ext: &mut FileExtManagerBuilder,
                   _url_scheme: &mut UrlSchemeManagerBuilder| { Ok(()) },
        )
        .await?;

        runtime
            .run(flows, async move |loader: &mut NodeLoader| {
                nodes(source, sink, loader).await
            })
            .await
    }
}
