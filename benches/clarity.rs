use std::time::Duration;

use blockstack_lib::clarity::vm::{contracts::Contract, database::ClaritySerializable, database::ClarityDeserializable};
use diesel::{insert_into, ExpressionMethods, RunQueryDsl};
use criterion::{criterion_group, Criterion};
use stacks_node_db_bench::utils::{apply_migrations, new_sqlite_db, random_bytes, random_string, tmp_file};
use lzzzz::lz4;

criterion_group! {
    name = insert_clarity_contract;
    config = Criterion::default().sample_size(500).measurement_time(Duration::from_secs(10));
    targets =
        insert_clarity_contract_current,
        insert_clarity_contract_next
}

fn main() {
    coredump::register_panic_handler().expect("Failed to register panic handler");

    insert_clarity_contract();

    Criterion::default()
        .configure_from_args()
        .final_summary();
}

fn insert_clarity_contract_current(c: &mut Criterion) {
    use stacks_node_db_bench::db_current::clarity::{
        DB_MIGRATIONS, CONTRACT_AST, CONTRACT_SOURCE,
        schema::metadata_table
    };

    let ast = Contract::deserialize(CONTRACT_AST).unwrap();
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    c.bench_function("contracts/current/insert", |b| {
        b.iter(|| {

            let serialized_ast = ast.serialize();

            insert_into(metadata_table::table)
                .values((
                    metadata_table::key.eq(random_string(64)),
                    metadata_table::blockhash.eq(random_string(64)),
                    metadata_table::value.eq(serialized_ast),
                ))
                .execute(&mut db)
                .unwrap();

            insert_into(metadata_table::table)
                .values((
                    metadata_table::key.eq(random_string(64)),
                    metadata_table::blockhash.eq(random_string(64)),
                    metadata_table::value.eq(CONTRACT_SOURCE),
                ))
                .execute(&mut db)
                .unwrap();

            // Simulate `contract-data-size`
            insert_into(metadata_table::table)
                .values((
                    metadata_table::key.eq(random_string(64)),
                    metadata_table::blockhash.eq(random_string(64)),
                    metadata_table::value.eq(1.to_string()),
                ))
                .execute(&mut db)
                .unwrap();

            // Simulate `contract-size`
            insert_into(metadata_table::table)
                .values((
                    metadata_table::key.eq(random_string(64)),
                    metadata_table::blockhash.eq(random_string(64)),
                    metadata_table::value.eq(1.to_string()),
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}

fn insert_clarity_contract_next(c: &mut Criterion) {
    use stacks_node_db_bench::db_next::clarity::{
        DB_MIGRATIONS, CONTRACT_AST, CONTRACT_SOURCE,
        schema::contract
    };

    //let analysis = ContractAnalysis::deserialize(CONTRACT_ANALYSIS).unwrap();
    let ast = Contract::deserialize(CONTRACT_AST).unwrap();
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    c.bench_function("contracts/next/insert", |b| {
        b.iter(|| {
            let serialized_ast = rmp_serde::to_vec(&ast).unwrap();

            let mut compressed_src = Vec::<u8>::with_capacity(10000);
            lzzzz::lz4::compress_to_vec(
                CONTRACT_SOURCE.as_bytes(), 
                &mut compressed_src, 
                lz4::ACC_LEVEL_DEFAULT
            ).unwrap();
            
            insert_into(contract::table)
                .values((
                    contract::contract_issuer.eq(random_string(32)),
                    contract::contract_name.eq(random_string(20)),
                    contract::block_hash.eq(random_bytes(32)),
                    contract::source_code.eq(compressed_src),
                    contract::data_size.eq(1),
                    contract::contract_size.eq(1),
                    contract::ast.eq(serialized_ast),
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}