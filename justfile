set dotenv-load

bench *args:
    RUST_BACKTRACE=1 cargo criterion --bench {{args}} --message-format=json | criterion-table > BENCHMARKS.md