# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [maps](#maps)

## Benchmark Results

### maps

|                      | `btree`                   | `hashmap (std)`                  | `hashmap (hashbrown)`            | `indexmap`                       | `hashmap (ahash)`                | `hashmap (fxhash)`                |
|:---------------------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`insert`**         | `578.14 us` (✅ **1.00x**) | `597.55 us` (✅ **1.03x slower**) | `554.19 us` (✅ **1.04x faster**) | `526.45 us` (✅ **1.10x faster**) | `526.39 us` (✅ **1.10x faster**) | `526.75 us` (✅ **1.10x faster**)  |
| **`to sorted vec`**  | `39.78 us` (✅ **1.00x**)  | `74.85 us` (❌ *1.88x slower*)    | `76.16 us` (❌ *1.91x slower*)    | `98.38 us` (❌ *2.47x slower*)    | `91.24 us` (❌ *2.29x slower*)    | `87.07 us` (❌ *2.19x slower*)     |
| **`random lookups`** | `104.03 us` (✅ **1.00x**) | `35.23 us` (🚀 **2.95x faster**)  | `19.16 us` (🚀 **5.43x faster**)  | `34.37 us` (🚀 **3.03x faster**)  | `17.16 us` (🚀 **6.06x faster**)  | `16.13 us` (🚀 **6.45x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

