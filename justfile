static:
    cargo run --release --bin static

dyn:
    cargo build --release -p sink --features "cdylib"
    cargo build --release -p source --features "cdylib"
    cargo run --release --bin dyn

static-dyn:
    cargo build --release -p sink --features "cdylib"
    cargo run --release --bin static_dyn

dyn-static:
    cargo build --release -p source --features "cdylib"
    cargo run --release --bin dyn_static

raw-static:
    cargo run --release --bin static --features "raw"

raw-dyn:
    cargo build --release -p sink --features "cdylib,raw"
    cargo build --release -p source --features "cdylib,raw"
    cargo run --release --bin dyn --features "raw"

raw-static-dyn:
    cargo build --release -p sink --features "cdylib,raw"
    cargo run --release --bin static_dyn --features "raw"

raw-dyn-static:
    cargo build --release -p source --features "cdylib,raw"
    cargo run --release --bin dyn_static --features "raw"

draw:
    uv --directory draw run draw

bench:  static \
        raw-static \
        dyn \
        static-dyn \
        dyn-static \
        raw-dyn \
        raw-static-dyn \
        raw-dyn-static \
        draw
