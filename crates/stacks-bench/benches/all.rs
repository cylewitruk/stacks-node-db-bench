use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

pub mod clarity;
pub mod general;
pub mod serialization;

criterion_group! {
    name = insert_block_hash_group;
    config = Criterion::default().sample_size(10000).measurement_time(Duration::from_secs(10));
    targets =
        general::insert_block_hash_as_hex_1,
        general::insert_block_hash_as_binary_1,
        general::insert_block_hash_as_hex_10,
        general::insert_block_hash_as_binary_10
}

criterion_group! {
    name = select_block_hash_group;
    config = Criterion::default().sample_size(10000).measurement_time(Duration::from_secs(10));
    targets =
        general::select_block_hash_as_hex_1,
        general::select_block_hash_as_binary_1,
        general::select_block_hash_as_hex_10,
        general::select_block_hash_as_binary_10
}

criterion_group! {
    name = ast_serialization;
    config = Criterion::default().sample_size(10000).measurement_time(Duration::from_secs(5));
    targets =
        serialization::serialize_ast_json,
        serialization::serialize_ast_msgpack,
        serialization::serialize_ast_speedy
}

criterion_group! {
    name = analysis_serialization;
    config = Criterion::default().sample_size(10000).measurement_time(Duration::from_secs(5));
    targets =
        serialization::serialize_analysis_json,
        serialization::serialize_analysis_msgpack,
        serialization::serialize_analysis_speedy
}

criterion_group! {
    name = insert_clarity_contract;
    config = Criterion::default().sample_size(10000).measurement_time(Duration::from_secs(15));
    targets =
        clarity::insert_clarity_contract_next,
        clarity::insert_clarity_contract_optimized
}

criterion_group! {
    name = select_clarity_contract;
    config = Criterion::default().sample_size(10000).measurement_time(Duration::from_secs(15));
    targets =
        clarity::select_clarity_contract_next,
        clarity::select_clarity_contract_optimized
}

criterion_main!(
    insert_block_hash_group,
    select_block_hash_group,

    ast_serialization, 
    analysis_serialization,

    insert_clarity_contract,
    select_clarity_contract
);