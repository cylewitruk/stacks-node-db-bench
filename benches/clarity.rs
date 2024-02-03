use std::time::Duration;

use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use criterion::{criterion_group, Criterion};
use rand::{thread_rng, Rng};
use ring::digest::{digest, Digest, SHA256};
use stacks_node_db_bench::utils::{apply_migrations, new_sqlite_db, random_string, tmp_file};

fn main() {
    coredump::register_panic_handler().expect("Failed to register panic handler");

    //insert_block_hash_group();
    //select_block_hash_group();

    Criterion::default()
        .configure_from_args()
        .final_summary();
}

fn insert_clarity_contract_current(c: &mut Criterion) {
    c.bench_function("contracts/current/insert", |b| {

        let tmpfile = tmp_file();
        let mut db = new_sqlite_db(&tmpfile);

        

        b.iter(|| {
            // Insert contract data into the db_current clarity database
            // Your code here...
        });
    });
}