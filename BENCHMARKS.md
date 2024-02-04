# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [serialization](#serialization)

## Benchmark Results

### serialization

|                | `json`                    | `msgpack`                         |
|:---------------|:--------------------------|:--------------------------------- |
| **`ast`**      | `254.94 us` (✅ **1.00x**) | `166.26 us` (✅ **1.53x faster**)  |
| **`analysis`** | `53.31 us` (✅ **1.00x**)  | `32.98 us` (✅ **1.62x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

