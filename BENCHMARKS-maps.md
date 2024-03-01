# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [maps](#maps)

## Benchmark Results

### maps

|                      | `btree`                   | `hashmap (std)`                  | `hashmap (hashbrown)`            | `indexmap`                        |
|:---------------------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`insert`**         | `270.82 us` (✅ **1.00x**) | `257.39 us` (✅ **1.05x faster**) | `217.97 us` (✅ **1.24x faster**) | `222.85 us` (✅ **1.22x faster**)  |
| **`to sorted vec`**  | `27.92 us` (✅ **1.00x**)  | `65.64 us` (❌ *2.35x slower*)    | `64.25 us` (❌ *2.30x slower*)    | `64.77 us` (❌ *2.32x slower*)     |
| **`random lookups`** | `84.81 us` (✅ **1.00x**)  | `32.41 us` (🚀 **2.62x faster**)  | `16.73 us` (🚀 **5.07x faster**)  | `34.55 us` (🚀 **2.45x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

