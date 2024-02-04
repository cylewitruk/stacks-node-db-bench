use std::time::Duration;

use blockstack_lib::clarity::vm::{contracts::Contract, database::ClaritySerializable, database::ClarityDeserializable};
use diesel::{insert_into, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use criterion::{criterion_group, Criterion};
use rand::{thread_rng, Rng};
use stacks_node_db_bench::utils::{apply_migrations, new_sqlite_db, random_bytes, random_string, tmp_file};
use lzzzz::lz4;

criterion_group! {
    name = insert_clarity_contract;
    config = Criterion::default().sample_size(2000).measurement_time(Duration::from_secs(10));
    targets =
        insert_clarity_contract_current,
        insert_clarity_contract_next
}

criterion_group! {
    name = select_clarity_contract;
    config = Criterion::default().sample_size(2000).measurement_time(Duration::from_secs(10));
    targets =
        select_clarity_contract_current,
        select_clarity_contract_next
}

fn main() {
    coredump::register_panic_handler().expect("Failed to register panic handler");

    insert_clarity_contract();
    select_clarity_contract();

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

            let mut compressed_ast = Vec::<u8>::with_capacity(10000);
            lzzzz::lz4::compress_to_vec(
                &serialized_ast, 
                &mut compressed_ast, 
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
                    contract::ast.eq(&compressed_ast),
                    contract::ast_size.eq(serialized_ast.len() as i32)
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}

fn select_clarity_contract_current(c: &mut Criterion) {
    use stacks_node_db_bench::db_current::clarity::{
        DB_MIGRATIONS, CONTRACT_AST, CONTRACT_SOURCE,
        schema::metadata_table
    };

    let ast = Contract::deserialize(CONTRACT_AST).unwrap();
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let serialized_ast = ast.serialize();

    let mut keys = Vec::<(String, String)>::with_capacity(1000);
    for _ in 0..1000 {
        let key = random_string(64);
        let blockhash = random_string(64);

        keys.push((key.clone(), blockhash.clone()));

        let mut contract_key = key.clone();
        contract_key.push_str("::contract");
        insert_into(metadata_table::table)
            .values((
                metadata_table::key.eq(contract_key),
                metadata_table::blockhash.eq(&blockhash),
                metadata_table::value.eq(&serialized_ast),
            ))
            .execute(&mut db)
            .unwrap();

        let mut source_key = key.clone();
        source_key.push_str("::contract-src");
        insert_into(metadata_table::table)
            .values((
                metadata_table::key.eq(source_key),
                metadata_table::blockhash.eq(&blockhash),
                metadata_table::value.eq(CONTRACT_SOURCE),
            ))
            .execute(&mut db)
            .unwrap();

        // Simulate `contract-data-size`
        let mut data_size_key = key.clone();
        data_size_key.push_str("::contract-data-size");
        insert_into(metadata_table::table)
            .values((
                metadata_table::key.eq(data_size_key),
                metadata_table::blockhash.eq(&blockhash),
                metadata_table::value.eq(1.to_string()),
            ))
            .execute(&mut db)
            .unwrap();

        // Simulate `contract-size`
        let mut contract_size_key = key.clone();
        contract_size_key.push_str("::contract-size");
        insert_into(metadata_table::table)
            .values((
                metadata_table::key.eq(contract_size_key),
                metadata_table::blockhash.eq(&blockhash),
                metadata_table::value.eq(1.to_string()),
            ))
            .execute(&mut db)
            .unwrap();
    }

    let idx = || -> usize {
        thread_rng().gen_range(0..keys.len())
    };

    c.bench_function("contracts/current/select", |b| {
        b.iter(|| {
            let key = &keys[idx()];

            // Contract AST
            let mut contract_key = key.clone();
            contract_key.0.push_str("::contract");
            let ast = metadata_table::table
                .select((
                    metadata_table::key,
                    metadata_table::blockhash,
                    metadata_table::value
                ))
                .filter(
                    metadata_table::key.eq(&contract_key.0)
                        .and(metadata_table::blockhash.eq(&contract_key.1))
                )
                .first::<(String, String, String)>(&mut db)
                .unwrap();
            assert_eq!(ast.2, serialized_ast);
            Contract::deserialize(&ast.2).unwrap();

            // Contract source
            let mut source_key = key.clone();
            source_key.0.push_str("::contract-src");
            let src = metadata_table::table
                .select((
                    metadata_table::key,
                    metadata_table::blockhash,
                    metadata_table::value
                ))
                .filter(
                    metadata_table::key.eq(&source_key.0)
                        .and(metadata_table::blockhash.eq(&source_key.1))
                )
                .first::<(String, String, String)>(&mut db)
                .unwrap();
            assert_eq!(src.2, CONTRACT_SOURCE);

            // Simulate `contract-data-size`
            let mut data_size_key = key.clone();
            data_size_key.0.push_str("::contract-data-size");
            let _ = metadata_table::table
                .select((
                    metadata_table::key,
                    metadata_table::blockhash,
                    metadata_table::value
                ))
                .filter(
                    metadata_table::key.eq(&data_size_key.0)
                        .and(metadata_table::blockhash.eq(&data_size_key.1))
                )
                .first::<(String, String, String)>(&mut db)
                .unwrap();

            // Simulate `contract-size`
            let mut contract_size_key = key.clone();
            contract_size_key.0.push_str("::contract-size");
            let _ = metadata_table::table
                .select((
                    metadata_table::key,
                    metadata_table::blockhash,
                    metadata_table::value
                ))
                .filter(
                    metadata_table::key.eq(&contract_size_key.0)
                        .and(metadata_table::blockhash.eq(&contract_size_key.1))
                )
                .first::<(String, String, String)>(&mut db)
                .unwrap();
        });
    });
}

fn select_clarity_contract_next(c: &mut Criterion) {
    use stacks_node_db_bench::db_next::clarity::{
        DB_MIGRATIONS, CONTRACT_AST, CONTRACT_SOURCE,
        schema::contract
    };

    let ast = Contract::deserialize(CONTRACT_AST).unwrap();
    let ast = rmp_serde::to_vec(&ast).unwrap();
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut keys = Vec::<(String, String, Vec<u8>)>::with_capacity(1000);

    let mut compressed_src = Vec::<u8>::with_capacity(10000);
    lzzzz::lz4::compress_to_vec(
        CONTRACT_SOURCE.as_bytes(), 
        &mut compressed_src, 
        lz4::ACC_LEVEL_DEFAULT
    ).unwrap();

    let mut compressed_ast = Vec::<u8>::with_capacity(10000);
    lzzzz::lz4::compress_to_vec(
        &ast, 
        &mut compressed_ast, 
        lz4::ACC_LEVEL_DEFAULT
    ).unwrap();
    
    //eprintln!("Uncompressed source size: {}, Compressed source size: {}", CONTRACT_SOURCE.as_bytes().len(), compressed_src.len());

    for _ in 0..1000 {
        let contract_issuer = random_string(32);
        let contract_name = random_string(20);
        let block_hash = random_bytes(32);

        keys.push((contract_issuer.clone(), contract_name.clone(), block_hash.clone()));

        insert_into(contract::table)
            .values((
                contract::contract_issuer.eq(&contract_issuer),
                contract::contract_name.eq(&contract_name),
                contract::block_hash.eq(&block_hash),
                contract::source_code.eq(&compressed_src),
                contract::data_size.eq(1),
                contract::contract_size.eq(CONTRACT_SOURCE.len() as i32),
                contract::ast.eq(&compressed_ast),
                contract::ast_size.eq(ast.len() as i32)
            ))
            .execute(&mut db)
            .unwrap();
    }

    let idx = || -> usize {
        thread_rng().gen_range(0..keys.len())
    };

    c.bench_function("contracts/next/select", |b| {
        b.iter(|| {
            let key = &keys[idx()];

            let contract = contract::table
                .filter(
                    contract::contract_issuer.eq(&key.0)
                        .and(contract::contract_name.eq(&key.1))
                        .and(contract::block_hash.eq(&key.2))
                )
                .first::<stacks_node_db_bench::db_next::clarity::Contract>(&mut db)
                .unwrap();

            //eprintln!("Expected source size: {}, compressed size: {}, contract size: {}", CONTRACT_SOURCE.as_bytes().len(), compressed_src.len(), contract.contract_size);
            let mut contract_src = vec![0; contract.contract_size as usize];
            lz4::decompress(&contract.source_code, &mut contract_src).unwrap();
            assert_eq!(String::from_utf8(contract_src).unwrap(), CONTRACT_SOURCE);

            let mut decomp_ast = vec![0; contract.ast_size as usize];
            lz4::decompress(&contract.ast, &mut decomp_ast).unwrap();
            assert_eq!(ast, decomp_ast);

            rmp_serde::from_slice::<Contract>(&decomp_ast).unwrap();
        });
    });
}