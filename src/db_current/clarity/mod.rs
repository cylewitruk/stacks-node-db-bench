use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use self::schema::metadata_table;

pub mod schema;

// Embed our database migrations at compile-time so that they can easily be
// run at applicaton execution without needing external SQL files.
pub const DB_MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db_current/clarity/migrations");

pub const CONTRACT_ANALYSIS: &str = include_str!("data/contract_analysis.json");
pub const CONTRACT_AST: &str = include_str!("data/contract_ast.json");
pub const CONTRACT_SOURCE: &str = include_str!("data/contract_src.clar");

#[derive(Queryable, Selectable, Identifiable, PartialEq, Eq, Debug, Clone, QueryableByName)]
#[diesel(primary_key(key, blockhash))]
#[diesel(table_name = metadata_table)]
pub struct MetaDataEntry {
    pub key: String,
    pub blockhash: String,
    pub value: String,
}