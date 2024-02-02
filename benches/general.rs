use std::time::Duration;

use diesel::connection::SimpleConnection;
use diesel::{insert_into, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use criterion::{criterion_group, Criterion};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use ring::digest::{digest, Digest, SHA256};
use stacks_node_db_bench::db_general::schema::block_hash_as_binary::id;
use tempfile::NamedTempFile;

criterion_group! {
    name = insert_block_hash_group;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets =
        insert_block_hash_as_hex,
        insert_block_hash_as_binary
}

criterion_group! {
    name = select_block_hash_group;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets =
        select_block_hash_as_hex,
        select_block_hash_as_binary
}

fn main() {
    coredump::register_panic_handler().expect("Failed to register panic handler");

    insert_block_hash_group();
    select_block_hash_group();

    Criterion::default()
        .configure_from_args()
        .final_summary();
}

/// Applies the provided db migrations to the provided db.
fn apply_migrations(db: &mut SqliteConnection, migrations: EmbeddedMigrations) {

    db.batch_execute("
        PRAGMA journal_mode = WAL;          -- better write-concurrency
        PRAGMA synchronous = NORMAL;        -- fsync only in critical moments
        PRAGMA wal_autocheckpoint = 1000;   -- write WAL changes back every 1000 pages, for an in average 1MB WAL file. May affect readers if number is increased
        PRAGMA wal_checkpoint(TRUNCATE);    -- free some space by truncating possibly massive WAL files from the last run.
        PRAGMA busy_timeout = 250;          -- sleep if the database is busy
        PRAGMA foreign_keys = ON;           -- enforce foreign keys
    ").unwrap();

    MigrationHarness::run_pending_migrations(db, migrations).unwrap();
}

/// Generates a random string of length n.
fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(n)
        .collect()
}

fn tmp_file() -> NamedTempFile {
    tempfile::Builder::new().prefix("db-bench").suffix(".sqlite").tempfile().unwrap()
}

fn new_sqlite_db(db_file: &NamedTempFile) -> SqliteConnection {
    SqliteConnection::establish(&db_file.path().to_str().unwrap()).unwrap()
}

fn insert_block_hash_as_hex(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_text};
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut samples = Vec::<String>::new();

    for _ in 0..1000{
        let random_string = random_string(64);
        let hash = ring::digest::digest(&SHA256, random_string.as_bytes());

        samples.push(hex::encode(hash));
    }

    c.bench_function("general/block_hash_as_hex/insert", |b| {
        b.iter(|| {
            let idx = thread_rng().gen_range(0..samples.len());
            insert_into(block_hash_as_text::table)
                .values((
                    block_hash_as_text::block_hash.eq(&samples[idx]),
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}

fn insert_block_hash_as_binary(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_binary};

    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);
    
    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut samples = Vec::<Digest>::new();

    for _ in 0..1000 {
        let random_string = random_string(64);
        let hash = ring::digest::digest(&SHA256, random_string.as_bytes());

        samples.push(hash);
    }

    c.bench_function("general/block_hash_as_binary/insert", |b| {
        b.iter(|| {
            insert_into(block_hash_as_binary::table)
                .values((
                    block_hash_as_binary::block_hash.eq(samples[thread_rng().gen_range(0..samples.len())].as_ref()),
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}

fn select_block_hash_as_hex(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_text};
    
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

        insert_into(block_hash_as_text::table)
            .values((
                block_hash_as_text::block_hash.eq(hex::encode(hash)),
            ))
            .execute(&mut db)
            .unwrap();
    }

    c.bench_function("general/block_hash_as_hex/select", |b| {
        b.iter(|| {
            let idx = thread_rng().gen_range(0..samples.len());
            block_hash_as_text::table
                .filter(block_hash_as_text::block_hash.eq(hex::encode(samples[idx])))
                .select(block_hash_as_text::block_hash)
                .load::<String>(&mut db)
                .unwrap();
        });
    });
}

fn select_block_hash_as_binary(c: &mut Criterion) {
    use stacks_node_db_bench::db_general::{DB_MIGRATIONS, schema::block_hash_as_binary};
    
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

        insert_into(block_hash_as_binary::table)
            .values((
                block_hash_as_binary::block_hash.eq(hash.as_ref()),
            ))
            .execute(&mut db)
            .unwrap();
    }

    c.bench_function("general/block_hash_as_binary/select", |b| {
        b.iter(|| {
            let idx = thread_rng().gen_range(0..samples.len());
            block_hash_as_binary::table
                .filter(block_hash_as_binary::block_hash.eq(samples[idx].as_ref()))
                .select(block_hash_as_binary::block_hash)
                .load::<Vec<u8>>(&mut db)
                .unwrap();
        });
    });
}