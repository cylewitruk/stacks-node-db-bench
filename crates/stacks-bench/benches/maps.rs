use std::collections::BTreeMap;

use criterion::{criterion_group, Criterion};

use itertools::Itertools;
use indexmap::IndexMap;
use rand::{thread_rng, Rng};
use stacks_node_db_bench::utils::{random_string, random_bytes};

criterion_group!(
    maps_insert, btreemap_insert, hashmap_insert_std, hashmap_insert_hashbrown, 
    indexmap_insert, hashmap_insert_ahash, hashmap_insert_fxhash
);

criterion_group!(
    maps_to_vec_sorted, btreemap_to_vec_sorted, hashmap_to_vec_sorted_std, 
    hashmap_to_vec_sorted_hashbrown, indexmap_to_vec_sorted, hashmap_to_vec_sorted_ahash, 
    hashmap_to_vec_sorted_fxhash
);

criterion_group!(maps_random_lookups, btreemap_random_lookups, hashmap_random_lookups_std, 
    hashmap_random_lookups_hashbrown, indexmap_random_lookups, hashmap_random_lookups_ahash, 
    hashmap_random_lookups_fxhash
);

fn main() {
    coredump::register_panic_handler().expect("Failed to register panic handler");

    maps_insert();
    maps_to_vec_sorted();
    maps_random_lookups();

    Criterion::default()
        .configure_from_args()
        .final_summary();
}

pub fn btreemap_insert(c: &mut Criterion) {
    c.bench_function("maps/btree/insert", |b| {
        b.iter_batched(|| {
            BTreeMap::<String, Vec<u8>>::new()
        },
        |mut map| {
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_insert_std(c: &mut Criterion) {
    use std::collections::HashMap;

    c.bench_function("maps/hashmap (std)/insert", |b| {
        b.iter_batched(|| HashMap::<String, Vec<u8>>::new(),
        |mut map| {
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_insert_hashbrown(c: &mut Criterion) {
    use hashbrown::HashMap;

    c.bench_function("maps/hashmap (hashbrown)/insert", |b| {
        b.iter_batched(|| HashMap::<String, Vec<u8>>::new(), 
        |mut map| {
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
        }, 
        criterion::BatchSize::SmallInput);
    });
}

pub fn indexmap_insert(c: &mut Criterion) {
    c.bench_function("maps/indexmap/insert", |b| {
        b.iter_batched(|| IndexMap::<String, Vec<u8>>::new(), 
        |mut map| {
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
        }, 
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_insert_ahash(c: &mut Criterion) {
    c.bench_function("maps/hashmap (ahash)/insert", |b| {
        b.iter_batched(|| std::collections::HashMap::<String, Vec<u8>, ahash::RandomState>::default(),
        |mut map| {
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
        }, 
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_insert_fxhash(c: &mut Criterion) {
    c.bench_function("maps/hashmap (fxhash)/insert", |b| {
        b.iter_batched(|| std::collections::HashMap::<String, Vec<u8>, fxhash::FxBuildHasher>::default(),
        |mut map| {
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
        }, 
        criterion::BatchSize::SmallInput);
    });
}

pub fn btreemap_to_vec_sorted(c: &mut Criterion) {
    c.bench_function("maps/btree/to sorted vec", |b| {
        b.iter_batched(|| {
            let mut map: BTreeMap<String, Vec<u8>> = BTreeMap::new();
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
            map
        },
        |map| {
            let _ = map.iter().collect::<Vec<_>>();
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_to_vec_sorted_std(c: &mut Criterion) {
    c.bench_function("maps/hashmap (std)/to sorted vec", |b| {

        b.iter_batched(|| {
            let mut map: std::collections::HashMap<String, Vec<u8>> = std::collections::HashMap::new();
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
            map
        },
        |map| {
            let _ = map.iter()
                .sorted_unstable_by_key(|(key, _)| *key)
                .collect::<Vec<_>>();
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_to_vec_sorted_hashbrown(c: &mut Criterion) {
    c.bench_function("maps/hashmap (hashbrown)/to sorted vec", |b| {
        b.iter_batched(|| {
            let mut map: hashbrown::HashMap<String, Vec<u8>> = hashbrown::HashMap::new();
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
            map
        },
        |map| {
            let _ = map.iter()
                .sorted_unstable_by_key(|(key, _)| *key)
                .collect::<Vec<_>>();
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn indexmap_to_vec_sorted(c: &mut Criterion) {
    c.bench_function("maps/indexmap/to sorted vec", |b| {
        b.iter_batched(|| {
            let mut map: IndexMap<String, Vec<u8>> = IndexMap::new();
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
            map
        },
        |mut map| {
            map.sort_unstable_keys();
            let _ = map.iter().collect_vec();
            //let _ = map.sorted_unstable_by(|key1, _, key2, _| key1.cmp(key2))
            //    .collect::<Vec<_>>();
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_to_vec_sorted_ahash(c: &mut Criterion) {
    c.bench_function("maps/hashmap (ahash)/to sorted vec", |b| {
        b.iter_batched(|| {
            let mut map: std::collections::HashMap<String, Vec<u8>, ahash::RandomState> = std::collections::HashMap::default();
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
            map
        },
        |map| {
            let _ = map.iter()
                .sorted_unstable_by_key(|(key, _)| *key)
                .collect::<Vec<_>>();
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_to_vec_sorted_fxhash(c: &mut Criterion) {
    c.bench_function("maps/hashmap (fxhash)/to sorted vec", |b| {
        b.iter_batched(|| {
            let mut map: std::collections::HashMap<String, Vec<u8>, fxhash::FxBuildHasher> = std::collections::HashMap::default();
            for _ in 0..1000 {
                map.insert(random_string(50), random_bytes(100));
            }
            map
        },
        |map| {
            let _ = map.iter()
                .sorted_unstable_by_key(|(key, _)| *key)
                .collect::<Vec<_>>();
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn btreemap_random_lookups(c: &mut Criterion) {
    let mut map: BTreeMap<String, Vec<u8>> = BTreeMap::new();

    let mut keys = Vec::<String>::new();

    for _ in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), random_bytes(100));
        keys.push(key);
    }

    c.bench_function("maps/btree/random lookups", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                map.get(&keys[thread_rng().gen_range(0..1000)]);
            }
        });
    });
}

pub fn hashmap_random_lookups_std(c: &mut Criterion) {
    let mut map: std::collections::HashMap<String, Vec<u8>> = std::collections::HashMap::new();
    let mut keys = Vec::<String>::new();

    for _ in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), random_bytes(100));
        keys.push(key);
    }

    c.bench_function("maps/hashmap (std)/random lookups", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                map.get(&keys[thread_rng().gen_range(0..1000)]);
            }
        });
    });
}

pub fn hashmap_random_lookups_hashbrown(c: &mut Criterion) {
    let mut map: hashbrown::HashMap<String, Vec<u8>> = hashbrown::HashMap::new();
    let mut keys = Vec::<String>::new();

    for _ in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), random_bytes(100));
        keys.push(key);
    }

    c.bench_function("maps/hashmap (hashbrown)/random lookups", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                map.get(&keys[thread_rng().gen_range(0..1000)]);
            }
        });
    });
}

pub fn indexmap_random_lookups(c: &mut Criterion) {
    let mut map: IndexMap<String, Vec<u8>> = IndexMap::new();
    let mut keys = Vec::<String>::new();

    for _ in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), random_bytes(100));
        keys.push(key);
    }

    c.bench_function("maps/indexmap/random lookups", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                map.get(&keys[thread_rng().gen_range(0..1000)]);
            }
        });
    });
}

pub fn hashmap_random_lookups_ahash(c: &mut Criterion) {
    let mut map: std::collections::HashMap<String, Vec<u8>, ahash::RandomState> = std::collections::HashMap::default();
    let mut keys = Vec::<String>::new();

    for _ in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), random_bytes(100));
        keys.push(key);
    }

    c.bench_function("maps/hashmap (ahash)/random lookups", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                map.get(&keys[thread_rng().gen_range(0..1000)]);
            }
        });
    });
}

pub fn hashmap_random_lookups_fxhash(c: &mut Criterion) {
    let mut map: std::collections::HashMap<String, Vec<u8>, fxhash::FxBuildHasher> = std::collections::HashMap::default();
    let mut keys = Vec::<String>::new();

    for _ in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), random_bytes(100));
        keys.push(key);
    }

    c.bench_function("maps/hashmap (fxhash)/random lookups", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                map.get(&keys[thread_rng().gen_range(0..1000)]);
            }
        });
    });
}