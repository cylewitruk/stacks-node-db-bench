set dotenv-load

bench benchmark="all":
    RUST_BACKTRACE=1 cargo criterion --bench {{benchmark}} --message-format=json | criterion-table > {{ if benchmark == "all" { "BENCHMARKS.md" } else { "BENCHMARKS-" + benchmark + ".md" } }}