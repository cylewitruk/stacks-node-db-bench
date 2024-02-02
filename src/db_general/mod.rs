use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use self::schema::block_hash_as_binary;

pub mod schema;

// Embed our database migrations at compile-time so that they can easily be
// run at applicaton execution without needing external SQL files.
pub const DB_MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db_general/migrations");

#[derive(
    Queryable, Selectable, Identifiable, PartialEq, Eq, Debug, Clone, QueryableByName, Insertable,
)]
#[diesel(table_name = block_hash_as_binary)]
pub struct BlockHashAsBinary {
    pub id: i32,
    pub block_hash: Vec<u8>,
}

pub struct BlockHashAsHex {
    pub id: i32,
    pub block_hash: String,
}