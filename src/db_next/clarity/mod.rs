use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub mod schema;

// Embed our database migrations at compile-time so that they can easily be
// run at applicaton execution without needing external SQL files.
pub const DB_MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db_next/clarity/migrations");

pub const CONTRACT_ANALYSIS: &str = include_str!("../../../data/clarity/contract_analysis.json");
pub const CONTRACT_AST: &str = include_str!("../../../data/clarity/contract_ast.json");
pub const CONTRACT_SOURCE: &str = include_str!("../../../data/clarity/contract_src.clar");

pub struct Contract {
    pub contract_issuer: String,
    pub contract_name: String,
    pub block_hash: Vec<u8>,
    pub source_code: Vec<u8>,
    pub data_size: i32,
    pub contract_size: i32,
    pub ast: Vec<u8>,
}