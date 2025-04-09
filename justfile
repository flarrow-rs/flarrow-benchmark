static:
    cargo run --release --bin static --features no-dylib

dyn:
    cargo build --release -p sink
    cargo build --release -p source
    cargo run --release --bin dyn

static-dyn:
    cargo build --release -p sink
    cargo run --release --bin static_dyn --features source-no-dylib

dyn-static:
    cargo build --release -p source
    cargo run --release --bin dyn_static --features sink-no-dylib

raw-static:
    cargo run --release --bin static --features "no-dylib, raw"

raw-dyn:
    cargo build --release -p sink --features "raw"
    cargo build --release -p source --features "raw"
    cargo run --release --bin dyn

raw-static-dyn:
    cargo build --release -p sink --features "raw"
    cargo run --release --bin static_dyn --features "source-no-dylib, raw"

raw-dyn-static:
    cargo build --release -p source --features "raw"
    cargo run --release --bin dyn_static --features "sink-no-dylib, raw"
