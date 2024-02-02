use std::time::Duration;

use diesel::{Connection, SqliteConnection};
use criterion::{criterion_group, Criterion};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

criterion_group! {
    name = insert_block_hash;
    config = Criterion::default().measurement_time(Duration::from_secs(5));
    targets =
        insert_block_hash_as_hex,
        insert_block_hash_as_binary
}

fn main() {
    coredump::register_panic_handler().expect("Failed to register panic handler");

    insert_block_hash();

    Criterion::default()
        .configure_from_args()
        .final_summary();
}

/// Creates a new temporary database file.
fn tmpfile() -> String {
    tempfile::Builder::new()
        .prefix("db-bench")
        .suffix(".sqlite")
        .tempfile()
        .unwrap()
        .path()
        .to_str()
        .unwrap()
        .to_owned()
}

/// Opens and returns a connection to a new temporary SQLite database.
fn new_sqlite_db() -> SqliteConnection {
    let db_file = tmpfile();
    SqliteConnection::establish(&db_file).unwrap()
}

fn apply_migrations(db: &mut SqliteConnection, migrations: EmbeddedMigrations) {
    MigrationHarness::run_pending_migrations(db, migrations).unwrap();
}

fn insert_block_hash_as_hex(c: &mut Criterion) {
    let db = new_sqlite_db();

    c.bench_function("diesel", |b| {
        b.iter(|| {
        });
    });
}

fn insert_block_hash_as_binary(c: &mut Criterion) {
    c.bench_function("diesel", |b| {
        b.iter(|| {
        });
    });
}