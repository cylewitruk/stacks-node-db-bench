# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [serialization](#serialization)

## Benchmark Results

### serialization

|                | `json`                    | `msgpack`                        | `speedy`                        | `rkyv`                           |
|:---------------|:--------------------------|:---------------------------------|:--------------------------------|:-------------------------------- |
| **`ast`**      | `284.71 us` (✅ **1.00x**) | `194.76 us` (✅ **1.46x faster**) | `79.68 us` (🚀 **3.57x faster**) | `35.14 us` (🚀 **8.10x faster**)  |
| **`analysis`** | `52.33 us` (✅ **1.00x**)  | `33.21 us` (✅ **1.58x faster**)  | `17.62 us` (🚀 **2.97x faster**) | `9.84 us` (🚀 **5.32x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

