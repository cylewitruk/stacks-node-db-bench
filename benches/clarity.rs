use std::time::Duration;

use blockstack_lib::clarity::vm::{analysis::ContractAnalysis, contracts::Contract, database::{ClarityDeserializable, ClaritySerializable}};
use diesel::{insert_into, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use criterion::{criterion_group, Criterion};
use rand::{thread_rng, Rng};
use speedy::Readable;
use stacks_node_db_bench::utils::{
    apply_migrations, new_sqlite_db, random_bytes, random_string, tmp_file
};

criterion_group! {
    name = insert_clarity_contract;
    config = Criterion::default().sample_size(2000).measurement_time(Duration::from_secs(10));
    targets =
        insert_clarity_contract_next,
        insert_clarity_contract_optimized
}

criterion_group! {
    name = select_clarity_contract;
    config = Criterion::default().sample_size(2000).measurement_time(Duration::from_secs(10));
    targets =
        select_clarity_contract_next,
        select_clarity_contract_optimized
}

fn main() {
    coredump::register_panic_handler().expect("Failed to register panic handler");

    insert_clarity_contract();
    select_clarity_contract();

    Criterion::default()
        .configure_from_args()
        .final_summary();
}

pub fn insert_clarity_contract_next(c: &mut Criterion) {
    use stacks_node_db_bench::db_current::clarity::{
        DB_MIGRATIONS, CONTRACT_AST, CONTRACT_SOURCE, CONTRACT_ANALYSIS,
        schema::metadata_table
    };

    let ast = Contract::deserialize(CONTRACT_AST).unwrap();
    let analysis = ContractAnalysis::deserialize(CONTRACT_ANALYSIS).unwrap();
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    c.bench_function("contracts/next/insert", |b| {
        b.iter(|| {

            let serialized_ast = ast.serialize();
            let serialized_analysis = analysis.serialize();

            insert_into(metadata_table::table)
                .values((
                    metadata_table::key.eq(random_string(64)),
                    metadata_table::blockhash.eq(hex::encode(random_bytes(32))),
                    metadata_table::value.eq(serialized_ast),
                ))
                .execute(&mut db)
                .unwrap();

            insert_into(metadata_table::table)
                .values((
                    metadata_table::key.eq(random_string(64)),
                    metadata_table::blockhash.eq(hex::encode(random_bytes(32))),
                    metadata_table::value.eq(CONTRACT_SOURCE),
                ))
                .execute(&mut db)
                .unwrap();

            // Simulate `contract-data-size`
            insert_into(metadata_table::table)
                .values((
                    metadata_table::key.eq(random_string(64)),
                    metadata_table::blockhash.eq(hex::encode(random_bytes(32))),
                    metadata_table::value.eq(1.to_string()),
                ))
                .execute(&mut db)
                .unwrap();

            // Simulate `contract-size`
            insert_into(metadata_table::table)
                .values((
                    metadata_table::key.eq(random_string(64)),
                    metadata_table::blockhash.eq(hex::encode(random_bytes(32))),
                    metadata_table::value.eq(1.to_string()),
                ))
                .execute(&mut db)
                .unwrap();

            // Simulate `analysis`
            insert_into(metadata_table::table)
                .values((
                    metadata_table::key.eq(random_string(64)),
                    metadata_table::blockhash.eq(hex::encode(random_bytes(32))),
                    metadata_table::value.eq(serialized_analysis),
                ))
                .execute(&mut db)
                .unwrap();
        });
    });
}

pub fn insert_clarity_contract_optimized(c: &mut Criterion) {
    use stacks_node_db_bench::db_next::clarity::{
        DB_MIGRATIONS, CONTRACT_AST, CONTRACT_SOURCE, CONTRACT_ANALYSIS,
        schema::contract, schema::contract_analysis
    };

    let analysis = ContractAnalysis::deserialize(CONTRACT_ANALYSIS).unwrap();
    let ast = Contract::deserialize(CONTRACT_AST).unwrap();
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);
    let mut analysis_id = 0;

    c.bench_function("contracts/optimized/insert", |b| {
        b.iter(|| {
            let serialized_ast = speedy::Writable::write_to_vec(&ast)
                .expect("failed to serialize contract AST");
            let serialized_analysis = speedy::Writable::write_to_vec(&analysis)
                .expect("failed to serialize contract analysis");

            let compressed_src = lz4_flex::block::compress(CONTRACT_SOURCE.as_bytes());
            let compressed_ast = lz4_flex::block::compress(&serialized_ast);
            let compressed_analysis = lz4_flex::block::compress(&serialized_analysis);
            
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

            insert_into(contract_analysis::table)
                .values((
                    contract_analysis::contract_id.eq(analysis_id),
                    contract_analysis::analysis.eq(&compressed_analysis),
                    contract_analysis::analysis_size.eq(serialized_analysis.len() as i32)
                ))
                .execute(&mut db)
                .unwrap();

            analysis_id += 1;
        });
    });
}

pub fn select_clarity_contract_next(c: &mut Criterion) {
    use stacks_node_db_bench::db_current::clarity::{
        DB_MIGRATIONS, CONTRACT_AST, CONTRACT_SOURCE, CONTRACT_ANALYSIS,
        schema::metadata_table
    };

    let ast = Contract::deserialize(CONTRACT_AST).unwrap();
    let analysis = ContractAnalysis::deserialize(CONTRACT_ANALYSIS).unwrap();
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let serialized_ast = ast.serialize();
    let serialized_analysis = analysis.serialize();

    let mut keys = Vec::<(String, String)>::with_capacity(1000);
    for _ in 0..1000 {
        let key = random_string(64);
        let blockhash = hex::encode(random_bytes(32));

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

        // Simulate `analysis`
        let mut analysis_key = key.clone();
        analysis_key.push_str("::analysis");
        insert_into(metadata_table::table)
            .values((
                metadata_table::key.eq(analysis_key),
                metadata_table::blockhash.eq(&blockhash),
                metadata_table::value.eq(&serialized_analysis),
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
            hex::decode(&ast.1).unwrap();
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
            hex::decode(&ast.1).unwrap();

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
            hex::decode(&ast.1).unwrap();

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
            hex::decode(&ast.1).unwrap();

            let mut analysis_key = key.clone();
            analysis_key.0.push_str("::analysis");
            let analysis_result = metadata_table::table
                .select((
                    metadata_table::key,
                    metadata_table::blockhash,
                    metadata_table::value
                ))
                .filter(
                    metadata_table::key.eq(&analysis_key.0)
                        .and(metadata_table::blockhash.eq(&analysis_key.1))
                )
                .first::<(String, String, String)>(&mut db)
                .unwrap();
            let _ = ContractAnalysis::deserialize(&analysis_result.2).unwrap();
            hex::decode(&ast.1).unwrap();
        });
    });
}

pub fn select_clarity_contract_optimized(c: &mut Criterion) {
    use stacks_node_db_bench::db_next::clarity::{
        DB_MIGRATIONS, CONTRACT_AST, CONTRACT_SOURCE, CONTRACT_ANALYSIS,
        schema::contract, schema::contract_analysis
    };

    let ast = Contract::deserialize(CONTRACT_AST)
        .unwrap();
    let ast = speedy::Writable::write_to_vec(&ast)
        .expect("failed to serialize contract AST");

    let analysis = ContractAnalysis::deserialize(CONTRACT_ANALYSIS)
        .unwrap();
    let analysis = speedy::Writable::write_to_vec(&analysis)
        .expect("failed to serialize contract analysis");
    
    let tmp = tmp_file();
    let mut db = new_sqlite_db(&tmp);

    apply_migrations(&mut db, DB_MIGRATIONS);

    let mut keys = Vec::<(i32, String, String, Vec<u8>)>::with_capacity(1000);

    let compressed_src = lz4_flex::block::compress(CONTRACT_SOURCE.as_bytes());
    let compressed_ast = lz4_flex::block::compress(&ast);
    let compressed_analysis = lz4_flex::block::compress(&analysis);
    
    for i in 0..1000 {
        let contract_issuer = random_string(32);
        let contract_name = random_string(20);
        let block_hash = random_bytes(32);

        keys.push((i, contract_issuer.clone(), contract_name.clone(), block_hash.clone()));

        insert_into(contract::table)
            .values((
                contract::id.eq(i),
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

        insert_into(contract_analysis::table)
            .values((
                contract_analysis::contract_id.eq(i),
                contract_analysis::analysis.eq(&compressed_analysis),
                contract_analysis::analysis_size.eq(analysis.len() as i32)
            ))
            .execute(&mut db)
            .unwrap();
    }

    let idx = || -> usize {
        thread_rng().gen_range(0..keys.len())
    };

    c.bench_function("contracts/optimized/select", |b| {
        b.iter(|| {
            let key = &keys[idx()];

            let contract_result = contract::table
                .filter(
                    contract::contract_issuer.eq(&key.1)
                        .and(contract::contract_name.eq(&key.2))
                        .and(contract::block_hash.eq(&key.3))
                )
                .first::<stacks_node_db_bench::db_next::clarity::Contract>(&mut db)
                .unwrap();

            let analysis_result = contract_analysis::table
                .filter(contract_analysis::contract_id.eq(contract_result.id))
                .first::<stacks_node_db_bench::db_next::clarity::ContractAnalysis>(&mut db)
                .unwrap();

            let contract_src = lz4_flex::decompress(&contract_result.source_code, CONTRACT_SOURCE.len()).unwrap();
            assert_eq!(String::from_utf8(contract_src).unwrap(), CONTRACT_SOURCE);

            let decomp_ast = lz4_flex::decompress(&contract_result.ast, ast.len()).unwrap();
            assert_eq!(ast, decomp_ast);
            let _ = Contract::read_from_buffer(&decomp_ast).unwrap();

            let decomp_analysis = lz4_flex::decompress(&analysis_result.analysis, analysis_result.analysis_size as usize).unwrap();
            assert_eq!(analysis, decomp_analysis);
            let _ = ContractAnalysis::read_from_buffer(&decomp_analysis).unwrap();

        });
    });
}