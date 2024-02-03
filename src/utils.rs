use diesel::{connection::SimpleConnection, Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tempfile::NamedTempFile;

/// Applies the provided db migrations to the provided db.
pub fn apply_migrations(db: &mut SqliteConnection, migrations: EmbeddedMigrations) {

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
pub fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(n)
        .collect()
}

pub fn random_bytes(n: usize) -> Vec<u8> {
    thread_rng().sample_iter(&Alphanumeric).take(n).collect()
}

pub fn tmp_file() -> NamedTempFile {
    tempfile::Builder::new().prefix("db-bench").suffix(".sqlite").tempfile().unwrap()
}

pub fn new_sqlite_db(db_file: &NamedTempFile) -> SqliteConnection {
    SqliteConnection::establish(&db_file.path().to_str().unwrap()).unwrap()
}