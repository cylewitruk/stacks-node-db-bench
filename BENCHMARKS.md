# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [block hash](#block-hash)

## Benchmark Results

### block hash

|                         | `as hex`                 | `as binary`                      |
|:------------------------|:-------------------------|:-------------------------------- |
| **`insert 1 column`**   | `53.08 us` (✅ **1.00x**) | `31.88 us` (✅ **1.66x faster**)  |
| **`insert 10 columns`** | `54.60 us` (✅ **1.00x**) | `40.01 us` (✅ **1.36x faster**)  |
| **`select 1 column`**   | `1.80 us` (✅ **1.00x**)  | `1.78 us` (✅ **1.01x faster**)   |
| **`select 10 columns`** | `8.37 us` (✅ **1.00x**)  | `7.54 us` (✅ **1.11x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

