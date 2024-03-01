use std::collections::BTreeMap;

use criterion::{criterion_group, Criterion};

use itertools::Itertools;
use rand::{thread_rng, Rng};
use stacks_node_db_bench::utils::random_string;

criterion_group!(maps_insert, btreemap_insert, hashmap_insert_std, hashmap_insert_hashbrown);
criterion_group!(maps_to_vec_sorted, btreemap_to_vec_sorted, hashmap_to_vec_sorted_std, hashmap_to_vec_sorted_hashbrown);
criterion_group!(maps_random_lookups, btreemap_random_lookups, hashmap_random_lookups_std, hashmap_random_lookups_hashbrown);

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
    let mut map: BTreeMap<String, i32> = BTreeMap::new();

    c.bench_function("maps/btree/insert", |b| {
        b.iter(|| {
            for i in 0..1000 {
                map.insert(random_string(50), i);
            }
        });
    });
}

pub fn hashmap_insert_std(c: &mut Criterion) {
    let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

    c.bench_function("maps/hashmap (std)/insert", |b| {
        b.iter(|| {
            for i in 0..1000 {
                map.insert(random_string(50), i);
            }
        });
    });
}

pub fn hashmap_insert_hashbrown(c: &mut Criterion) {
    let mut map: hashbrown::HashMap<String, i32> = hashbrown::HashMap::new();

    c.bench_function("maps/hashmap (hashbrown)/insert", |b| {
        b.iter(|| {
            for i in 0..1000 {
                map.insert(random_string(50), i);
            }
        });
    });
}

pub fn btreemap_to_vec_sorted(c: &mut Criterion) {
    let mut map: BTreeMap<String, i32> = BTreeMap::new();
    for i in 0..1000 {
        map.insert(random_string(50), i);
    }

    c.bench_function("maps/btree/to sorted vec", |b| {
        b.iter(|| {
            let _: Vec<_> = map.iter().collect();
        });
    });
}

pub fn hashmap_to_vec_sorted_std(c: &mut Criterion) {
    let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    for i in 0..1000 {
        map.insert(random_string(50), i);
    }

    c.bench_function("maps/hashmap (std)/to sorted vec", |b| {
        b.iter(|| {
            let _ = map.iter()
                .sorted_by_cached_key(|(key, _)| *key)
                .collect::<Vec<_>>();
        });
    });
}

pub fn hashmap_to_vec_sorted_hashbrown(c: &mut Criterion) {
    let mut map: hashbrown::HashMap<String, i32> = hashbrown::HashMap::new();
    for i in 0..1000 {
        map.insert(random_string(50), i);
    }

    c.bench_function("maps/hashmap (hashbrown)/to sorted vec", |b| {
        b.iter(|| {
            let _ = map.iter()
                .sorted_by_cached_key(|(key, _)| *key)
                .collect::<Vec<_>>();
        });
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