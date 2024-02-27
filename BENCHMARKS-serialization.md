# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [serialization](#serialization)

## Benchmark Results

### serialization

|                | `json`                    | `msgpack`                        | `cbor`                           | `speedy`                         |
|:---------------|:--------------------------|:---------------------------------|:---------------------------------|:-------------------------------- |
| **`ast`**      | `268.18 us` (✅ **1.00x**) | `165.36 us` (✅ **1.62x faster**) | `359.47 us` (❌ *1.34x slower*)   | `93.70 us` (🚀 **2.86x faster**)  |
| **`analysis`** | `64.31 us` (✅ **1.00x**)  | `38.35 us` (✅ **1.68x faster**)  | `95.45 us` (❌ *1.48x slower*)    | `22.66 us` (🚀 **2.84x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

