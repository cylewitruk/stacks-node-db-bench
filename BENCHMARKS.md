# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [general](#general)

## Benchmark Results

### general

|              | `block_hash_as_hex`          | `block_hash_as_binary`           |
|:-------------|:-----------------------------|:-------------------------------- |
| **`insert`** | `40.08 us` (✅ **1.00x**)     | `21.07 us` (🚀 **1.90x faster**)  |
| **`select`** | `2.07 us` (✅ **1.00x**)      | `1.68 us` (✅ **1.23x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

