run:
    cargo run
run_tracy:
    cargo run --release --features "tracy,debug"
run_rel:
    cargo run --release
build:
    cargo build
build_rel:
    cargo build --release
miri:
    cargo miri test -- --nocapture
test:
    cargo test -- --nocapture
bench:
    cargo bench --lib --quiet -- --color always --test-threads=1 --nocapture
clippy:
    cargo fmt
    cargo clippy
