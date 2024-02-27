use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use self::schema::{contract, contract_analysis};

pub mod schema;

// Embed our database migrations at compile-time so that they can easily be
// run at applicaton execution without needing external SQL files.
pub const DB_MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db_next/clarity/migrations");

pub const CONTRACT_ANALYSIS: &str = include_str!("../../../data/clarity/contract_analysis.json");
pub const CONTRACT_AST: &str = include_str!("../../../data/clarity/contract_ast.json");
pub const CONTRACT_SOURCE: &str = include_str!("../../../data/clarity/contract_src.clar");

#[derive(Queryable, Selectable, Identifiable, PartialEq, Eq, Debug, Clone, QueryableByName)]
#[diesel(primary_key(contract_issuer, contract_name, block_hash))]
#[diesel(table_name = contract)]
pub struct Contract {
    pub id: i32,
    pub contract_issuer: String,
    pub contract_name: String,
    pub block_hash: Vec<u8>,
    pub source_code: Vec<u8>,
    pub data_size: i32,
    pub contract_size: i32,
    pub ast: Vec<u8>,
    pub ast_size: i32
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Eq, Debug, Clone, QueryableByName)]
#[diesel(primary_key(contract_id))]
#[diesel(table_name = contract_analysis)]
pub struct ContractAnalysis {
    pub contract_id: i32,
    pub analysis: Vec<u8>,
    pub analysis_size: i32
}