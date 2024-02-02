use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub mod schema;

// Embed our database migrations at compile-time so that they can easily be
// run at applicaton execution without needing external SQL files.
pub const DB_MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db_current/chainstate/migrations");