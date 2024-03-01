use std::collections::BTreeMap;

use criterion::{criterion_group, Criterion};

use itertools::Itertools;
use indexmap::IndexMap;
use rand::{thread_rng, Rng};
use stacks_node_db_bench::utils::random_string;

criterion_group!(maps_insert, btreemap_insert, hashmap_insert_std, hashmap_insert_hashbrown, indexmap_insert);
criterion_group!(maps_to_vec_sorted, btreemap_to_vec_sorted, hashmap_to_vec_sorted_std, hashmap_to_vec_sorted_hashbrown, indexmap_to_vec_sorted);
criterion_group!(maps_random_lookups, btreemap_random_lookups, hashmap_random_lookups_std, hashmap_random_lookups_hashbrown, indexmap_random_lookups);

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
            BTreeMap::<String, i32>::new()
        },
        |mut map| {
            for i in 0..1000 {
                map.insert(random_string(50), i);
            }
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_insert_std(c: &mut Criterion) {
    use std::collections::HashMap;

    c.bench_function("maps/hashmap (std)/insert", |b| {
        b.iter_batched(|| HashMap::<String, i32>::new(),
        |mut map| {
            for i in 0..1000 {
                map.insert(random_string(50), i);
            }
        },
        criterion::BatchSize::SmallInput);
    });
}

pub fn hashmap_insert_hashbrown(c: &mut Criterion) {
    use hashbrown::HashMap;

    c.bench_function("maps/hashmap (hashbrown)/insert", |b| {
        b.iter_batched(|| HashMap::<String, i32>::new(), 
        |mut map| {
            for i in 0..1000 {
                map.insert(random_string(50), i);
            }
        }, 
        criterion::BatchSize::SmallInput);
    });
}

pub fn indexmap_insert(c: &mut Criterion) {
    let mut map: IndexMap<String, i32> = IndexMap::new();

    c.bench_function("maps/indexmap/insert", |b| {
        b.iter_batched(|| IndexMap::<String, i32>::new(), 
        |mut map| {
            for i in 0..1000 {
                map.insert(random_string(50), i);
            }
        }, 
        criterion::BatchSize::SmallInput);
    });
}

pub fn btreemap_to_vec_sorted(c: &mut Criterion) {
    c.bench_function("maps/btree/to sorted vec", |b| {
        b.iter_batched(|| {
            let mut map: BTreeMap<String, i32> = BTreeMap::new();
            for i in 0..1000 {
                map.insert(random_string(50), i);
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
            let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
            for i in 0..1000 {
                map.insert(random_string(50), i);
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
            let mut map: hashbrown::HashMap<String, i32> = hashbrown::HashMap::new();
            for i in 0..1000 {
                map.insert(random_string(50), i);
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
            let mut map: IndexMap<String, i32> = IndexMap::new();
            for i in 0..1000 {
                map.insert(random_string(50), i);
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

pub fn btreemap_random_lookups(c: &mut Criterion) {
    let mut map: BTreeMap<String, i32> = BTreeMap::new();

    let mut keys = Vec::<String>::new();

    for i in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), i);
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
    let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    let mut keys = Vec::<String>::new();

    for i in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), i);
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
    let mut map: hashbrown::HashMap<String, i32> = hashbrown::HashMap::new();
    let mut keys = Vec::<String>::new();

    for i in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), i);
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
    let mut map: IndexMap<String, i32> = IndexMap::new();
    let mut keys = Vec::<String>::new();

    for i in 0..1000 {
        let key = random_string(50);
        map.insert(key.clone(), i);
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