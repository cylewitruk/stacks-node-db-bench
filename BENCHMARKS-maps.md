# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [maps](#maps)

## Benchmark Results

### maps

|                      | `btree`                   | `hashmap (std)`                  | `hashmap (hashbrown)`            | `indexmap`                       | `hashmap (ahash)`                 |
|:---------------------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`insert`**         | `560.40 us` (✅ **1.00x**) | `554.46 us` (✅ **1.01x faster**) | `546.34 us` (✅ **1.03x faster**) | `556.71 us` (✅ **1.01x faster**) | `526.61 us` (✅ **1.06x faster**)  |
| **`to sorted vec`**  | `40.52 us` (✅ **1.00x**)  | `75.03 us` (❌ *1.85x slower*)    | `79.96 us` (❌ *1.97x slower*)    | `85.14 us` (❌ *2.10x slower*)    | `76.25 us` (❌ *1.88x slower*)     |
| **`random lookups`** | `78.38 us` (✅ **1.00x**)  | `29.73 us` (🚀 **2.64x faster**)  | `17.95 us` (🚀 **4.37x faster**)  | `31.24 us` (🚀 **2.51x faster**)  | `18.89 us` (🚀 **4.15x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

