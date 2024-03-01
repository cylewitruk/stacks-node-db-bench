# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [maps](#maps)

## Benchmark Results

### maps

|                      | `btree`                   | `hashmap (std)`                  | `hashmap (hashbrown)`            | `indexmap`                        |
|:---------------------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`insert`**         | `653.46 us` (✅ **1.00x**) | `611.74 us` (✅ **1.07x faster**) | `597.16 us` (✅ **1.09x faster**) | `581.33 us` (✅ **1.12x faster**)  |
| **`to sorted vec`**  | `43.13 us` (✅ **1.00x**)  | `78.96 us` (❌ *1.83x slower*)    | `71.72 us` (❌ *1.66x slower*)    | `79.61 us` (❌ *1.85x slower*)     |
| **`random lookups`** | `81.21 us` (✅ **1.00x**)  | `29.87 us` (🚀 **2.72x faster**)  | `16.08 us` (🚀 **5.05x faster**)  | `30.70 us` (🚀 **2.65x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

