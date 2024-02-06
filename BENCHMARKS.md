# Benchmarks

## Table of Contents

- [Benchmarks](#benchmarks)
  - [Table of Contents](#table-of-contents)
  - [Benchmark Results](#benchmark-results)
    - [block hash](#block-hash)
    - [serialization](#serialization)
    - [contracts](#contracts)

## Benchmark Results

### block hash

|                         | `as hex`                 | `as binary`                      |
|:------------------------|:-------------------------|:-------------------------------- |
| **`insert 1 column`**   | `45.92 us` (✅ **1.00x**) | `37.40 us` (✅ **1.23x faster**)  |
| **`insert 10 columns`** | `68.45 us` (✅ **1.00x**) | `37.14 us` (🚀 **1.84x faster**)  |
| **`select 1 column`**   | `1.71 us` (✅ **1.00x**)  | `1.61 us` (✅ **1.06x faster**)   |
| **`select 10 columns`** | `7.47 us` (✅ **1.00x**)  | `6.50 us` (✅ **1.15x faster**)   |

### serialization

|                | `json`                    | `msgpack+lz4`                         |
|:---------------|:--------------------------|:--------------------------------- |
| **`ast`**      | `262.90 us` (✅ **1.00x**) | `174.65 us` (✅ **1.51x faster**)  |
| **`analysis`** | `58.84 us` (✅ **1.00x**)  | `36.88 us` (✅ **1.60x faster**)   |

### contracts

|              | `current`                 | `next`                            |
|:-------------|:--------------------------|:--------------------------------- |
| **`insert`** | `447.93 us` (✅ **1.00x**) | `241.80 us` (🚀 **1.85x faster**)  |
| **`select`** | `242.16 us` (✅ **1.00x**) | `162.83 us` (✅ **1.49x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

