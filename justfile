set dotenv-load

bench *args:
    cargo criterion --bench {{args}} --message-format=json | criterion-table > BENCHMARKS.md