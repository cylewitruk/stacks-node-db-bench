# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [maps](#maps)

## Benchmark Results

### maps

|                      | `btree`                  | `hashmap (std)`                  | `hashmap (hashbrown)`             |
|:---------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`insert`**         | `1.34 ms` (✅ **1.00x**)  | `387.15 us` (🚀 **3.46x faster**) | `366.78 us` (🚀 **3.65x faster**)  |
| **`to sorted vec`**  | `3.18 us` (✅ **1.00x**)  | `36.53 us` (❌ *11.49x slower*)   | `34.45 us` (❌ *10.83x slower*)    |
| **`random lookups`** | `81.65 us` (✅ **1.00x**) | `28.73 us` (🚀 **2.84x faster**)  | `14.60 us` (🚀 **5.59x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

