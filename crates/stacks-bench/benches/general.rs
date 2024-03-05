use std::time::Duration;

use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use criterion::{criterion_group, Criterion};
use rand::{thread_rng, Rng};
use ring::digest::{digest, Digest, SHA256};
use stacks_node_db_bench::utils::{apply_migrations, new_sqlite_db, random_string, tmp_file};

criterion_group! {
    name = insert_block_hash_group;
    config = Criterion::default().measurement_time(Duration::from_secs(5));
    targets =
        insert_block_hash_as_hex_1,
        insert_block_hash_as_binary_1,
        insert_block_hash_as_hex_10,
        insert_block_hash_as_binary_10
}

criterion_group! {
    name = select_block_hash_group;
    config = Criterion::default().measurement_time(Duration::from_secs(5));
    targets =
        select_block_hash_as_hex_1,
        select_block_hash_as_binary_1,
        select_block_hash_as_hex_10,
        select_block_hash_as_binary_10
}

fn main() {
    coredump::register_panic_handler().expect("Failed to register panic handler");

    insert_block_hash_group();
    select_block_hash_group();

    Criterion::default()
        .configure_from_args()
        .final_summary();
}

pub fn insert_block_hash_as_hex_1(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_text_1};
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut samples = Vec::<String>::new();

    for _ in 0..1000{
        let random_string = random_string(64);
        let hash = ring::digest::digest(&SHA256, random_string.as_bytes());

        samples.push(hex::encode(hash));
    }

    c.bench_function("block hash/as hex/insert 1 column", |b| {
        b.iter(|| {
            let idx = thread_rng().gen_range(0..samples.len());
            insert_into(block_hash_as_text_1::table)
                .values((
                    block_hash_as_text_1::block_hash.eq(&samples[idx]),
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}

pub fn insert_block_hash_as_binary_1(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_binary_1};

    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);
    
    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut samples = Vec::<Digest>::new();

    for _ in 0..1000 {
        let random_string = random_string(64);
        let hash = ring::digest::digest(&SHA256, random_string.as_bytes());

        samples.push(hash);
    }

    c.bench_function("block hash/as binary/insert 1 column", |b| {
        b.iter(|| {
            insert_into(block_hash_as_binary_1::table)
                .values((
                    block_hash_as_binary_1::block_hash.eq(samples[thread_rng().gen_range(0..samples.len())].as_ref()),
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}

pub fn insert_block_hash_as_hex_10(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_text_10};
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut samples = Vec::<String>::new();

    for _ in 0..1000 {
        let random_string = random_string(64);
        let hash = ring::digest::digest(&SHA256, random_string.as_bytes());

        samples.push(hex::encode(hash));
    }

    let idx = || -> usize {
        thread_rng().gen_range(0..samples.len())
    };

    c.bench_function("block hash/as hex/insert 10 columns", |b| {
        b.iter(|| {
            insert_into(block_hash_as_text_10::table)
                .values((
                    block_hash_as_text_10::block_hash_1.eq(&samples[idx()]),
                    block_hash_as_text_10::block_hash_2.eq(&samples[idx()]),
                    block_hash_as_text_10::block_hash_3.eq(&samples[idx()]),
                    block_hash_as_text_10::block_hash_4.eq(&samples[idx()]),
                    block_hash_as_text_10::block_hash_5.eq(&samples[idx()]),
                    block_hash_as_text_10::block_hash_6.eq(&samples[idx()]),
                    block_hash_as_text_10::block_hash_7.eq(&samples[idx()]),
                    block_hash_as_text_10::block_hash_8.eq(&samples[idx()]),
                    block_hash_as_text_10::block_hash_9.eq(&samples[idx()]),
                    block_hash_as_text_10::block_hash_10.eq(&samples[idx()]),
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}

pub fn insert_block_hash_as_binary_10(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_binary_10};

    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);
    
    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut samples = Vec::<Digest>::new();

    for _ in 0..1000 {
        let random_string = random_string(64);
        let hash = ring::digest::digest(&SHA256, random_string.as_bytes());

        samples.push(hash);
    }

    let idx = || -> usize {
        thread_rng().gen_range(0..samples.len())
    };

    c.bench_function("block hash/as binary/insert 10 columns", |b| {
        b.iter(|| {
            insert_into(block_hash_as_binary_10::table)
                .values((
                    block_hash_as_binary_10::block_hash_1.eq(samples[idx()].as_ref()),
                    block_hash_as_binary_10::block_hash_2.eq(samples[idx()].as_ref()),
                    block_hash_as_binary_10::block_hash_3.eq(samples[idx()].as_ref()),
                    block_hash_as_binary_10::block_hash_4.eq(samples[idx()].as_ref()),
                    block_hash_as_binary_10::block_hash_5.eq(samples[idx()].as_ref()),
                    block_hash_as_binary_10::block_hash_6.eq(samples[idx()].as_ref()),
                    block_hash_as_binary_10::block_hash_7.eq(samples[idx()].as_ref()),
                    block_hash_as_binary_10::block_hash_8.eq(samples[idx()].as_ref()),
                    block_hash_as_binary_10::block_hash_9.eq(samples[idx()].as_ref()),
                    block_hash_as_binary_10::block_hash_10.eq(samples[idx()].as_ref()),
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}

pub fn select_block_hash_as_hex_1(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_text_1};
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut samples = Vec::<Digest>::new();

    for i in 0..150_000{
        let random_string = random_string(64);
        let hash = digest(&SHA256, random_string.as_bytes());

        if i % 1000 == 0 {
            samples.push(hash);
        }

        insert_into(block_hash_as_text_1::table)
            .values((
                block_hash_as_text_1::block_hash.eq(hex::encode(hash)),
            ))
            .execute(&mut db)
            .unwrap();
    }

    c.bench_function("block hash/as hex/select 1 column", |b| {
        b.iter(|| {
            let idx = thread_rng().gen_range(0..samples.len());
            block_hash_as_text_1::table
                .filter(block_hash_as_text_1::block_hash.eq(hex::encode(samples[idx])))
                .select(block_hash_as_text_1::block_hash)
                .load::<String>(&mut db)
                .unwrap();
        });
    });
}

pub fn select_block_hash_as_binary_1(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_binary_1};
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut samples = Vec::<Digest>::new();

    for i in 0..150_000{
        let random_string = random_string(64);
        let hash = digest(&SHA256, random_string.as_bytes());

        if i % 1000 == 0 {
            samples.push(hash);
        }

        insert_into(block_hash_as_binary_1::table)
            .values((
                block_hash_as_binary_1::block_hash.eq(hash.as_ref()),
            ))
            .execute(&mut db)
            .unwrap();
    }

    c.bench_function("block hash/as binary/select 1 column", |b| {
        b.iter(|| {
            let idx = thread_rng().gen_range(0..samples.len());
            block_hash_as_binary_1::table
                .filter(block_hash_as_binary_1::block_hash.eq(samples[idx].as_ref()))
                .select(block_hash_as_binary_1::block_hash)
                .load::<Vec<u8>>(&mut db)
                .unwrap();
        });
    });
}

pub fn select_block_hash_as_hex_10(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_text_10};
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut samples = Vec::<Digest>::new();

    for _ in 0..150_000 {
        let random_string = random_string(64);
        let hash = digest(&SHA256, random_string.as_bytes());

        samples.push(hash);
    }

    let idx = || -> usize {
        thread_rng().gen_range(0..samples.len())
    };

    let mut indexes = Vec::<Digest>::new();

    for _ in 0..150_000{
        // We need to keep track of the indexes we insert so we can select them later
        let index = samples[idx()];
        indexes.push(index);

        insert_into(block_hash_as_text_10::table)
            .values((
                block_hash_as_text_10::block_hash_1.eq(hex::encode(index)),
                block_hash_as_text_10::block_hash_2.eq(hex::encode(samples[idx()])),
                block_hash_as_text_10::block_hash_3.eq(hex::encode(samples[idx()])),
                block_hash_as_text_10::block_hash_4.eq(hex::encode(samples[idx()])),
                block_hash_as_text_10::block_hash_5.eq(hex::encode(samples[idx()])),
                block_hash_as_text_10::block_hash_6.eq(hex::encode(samples[idx()])),
                block_hash_as_text_10::block_hash_7.eq(hex::encode(samples[idx()])),
                block_hash_as_text_10::block_hash_8.eq(hex::encode(samples[idx()])),
                block_hash_as_text_10::block_hash_9.eq(hex::encode(samples[idx()])),
                block_hash_as_text_10::block_hash_10.eq(hex::encode(samples[idx()])),
            ))
            .execute(&mut db)
            .unwrap();
    }

    c.bench_function("block hash/as hex/select 10 columns", |b| {
        b.iter(|| {
            let idx = thread_rng().gen_range(0..indexes.len());
            block_hash_as_text_10::table
                .filter(block_hash_as_text_10::block_hash_1.eq(hex::encode(indexes[idx])))
                .select((
                    block_hash_as_text_10::block_hash_1,
                    block_hash_as_text_10::block_hash_2,
                    block_hash_as_text_10::block_hash_3,
                    block_hash_as_text_10::block_hash_4,
                    block_hash_as_text_10::block_hash_5,
                    block_hash_as_text_10::block_hash_6,
                    block_hash_as_text_10::block_hash_7,
                    block_hash_as_text_10::block_hash_8,
                    block_hash_as_text_10::block_hash_9,
                    block_hash_as_text_10::block_hash_10,
                ))
                .load::<(String, String, String, String, String, String, String, String, String, String)>(&mut db)
                .unwrap();
        });
    });
}

pub fn select_block_hash_as_binary_10(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_binary_10};
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    // Create our sample set of hashes
    let mut samples = Vec::<Digest>::new();
    for _ in 0..150_000 {
        let random_string = random_string(64);
        let hash = digest(&SHA256, random_string.as_bytes());

        samples.push(hash);
    }

    let mut indexes = Vec::<Digest>::new();

    let idx = || -> usize {
        thread_rng().gen_range(0..samples.len())
    };

    for _ in 0..150_000 {
        // We need to keep track of the indexes we insert so we can select them later
        let index = samples[idx()];
        indexes.push(index);

        insert_into(block_hash_as_binary_10::table)
            .values((
                block_hash_as_binary_10::block_hash_1.eq(index.as_ref()),
                block_hash_as_binary_10::block_hash_2.eq(samples[idx()].as_ref()),
                block_hash_as_binary_10::block_hash_3.eq(samples[idx()].as_ref()),
                block_hash_as_binary_10::block_hash_4.eq(samples[idx()].as_ref()),
                block_hash_as_binary_10::block_hash_5.eq(samples[idx()].as_ref()),
                block_hash_as_binary_10::block_hash_6.eq(samples[idx()].as_ref()),
                block_hash_as_binary_10::block_hash_7.eq(samples[idx()].as_ref()),
                block_hash_as_binary_10::block_hash_8.eq(samples[idx()].as_ref()),
                block_hash_as_binary_10::block_hash_9.eq(samples[idx()].as_ref()),
                block_hash_as_binary_10::block_hash_10.eq(samples[idx()].as_ref()),
            ))
            .execute(&mut db)
            .unwrap();
    }

    c.bench_function("block hash/as binary/select 10 columns", |b| {
        b.iter(|| {
            let idx = thread_rng().gen_range(0..indexes.len());
            block_hash_as_binary_10::table
                .filter(block_hash_as_binary_10::block_hash_1.eq(indexes[idx].as_ref()))
                .select((
                    block_hash_as_binary_10::block_hash_1,
                    block_hash_as_binary_10::block_hash_2,
                    block_hash_as_binary_10::block_hash_3,
                    block_hash_as_binary_10::block_hash_4,
                    block_hash_as_binary_10::block_hash_5,
                    block_hash_as_binary_10::block_hash_6,
                    block_hash_as_binary_10::block_hash_7,
                    block_hash_as_binary_10::block_hash_8,
                    block_hash_as_binary_10::block_hash_9,
                    block_hash_as_binary_10::block_hash_10,
                ))
                .load::<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>(&mut db)
                .unwrap();
        });
    });
}