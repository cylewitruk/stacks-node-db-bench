use std::time::Duration;

use blockstack_lib::clarity::vm::{analysis::ContractAnalysis, contracts::Contract, database::ClarityDeserializable, database::ClaritySerializable};
use criterion::{criterion_group, Criterion};

criterion_group! {
    name = ast_serialization;
    config = Criterion::default().sample_size(10000).measurement_time(Duration::from_secs(10));
    targets =
        serialize_ast_json,
        serialize_ast_msgpack
}

criterion_group! {
    name = analysis_serialization;
    config = Criterion::default().sample_size(10000).measurement_time(Duration::from_secs(10));
    targets =
        serialize_analysis_json,
        serialize_analysis_msgpack
}

fn main() {
    coredump::register_panic_handler().expect("Failed to register panic handler");

    ast_serialization();
    analysis_serialization();

    Criterion::default()
        .configure_from_args()
        .final_summary();
}

fn serialize_ast_json(c: &mut Criterion) {
    use stacks_node_db_bench::db_current::clarity::CONTRACT_AST;
    let ast = Contract::deserialize(CONTRACT_AST).unwrap();

    c.bench_function("serialization/json/ast", |b| {
        b.iter(|| {
            let serialized = ast.serialize();
            let _ = Contract::deserialize(&serialized);
        });
    });
}

fn serialize_ast_msgpack(c: &mut Criterion) {
    use stacks_node_db_bench::db_current::clarity::CONTRACT_AST;
    let ast = Contract::deserialize(CONTRACT_AST).unwrap();

    c.bench_function("serialization/msgpack/ast", |b| {
        b.iter(|| {
            let serialized = rmp_serde::to_vec(&ast).unwrap();
            rmp_serde::from_slice::<Contract>(&serialized).unwrap();
        });
    });
}

fn serialize_analysis_json(c: &mut Criterion) {
    use stacks_node_db_bench::db_current::clarity::CONTRACT_ANALYSIS;
    let analysis = ContractAnalysis::deserialize(CONTRACT_ANALYSIS).unwrap();

    c.bench_function("serialization/json/analysis", |b| {
        b.iter(|| {
            let serialized = analysis.serialize();
            let _ = ContractAnalysis::deserialize(&serialized);
        });
    });
}

fn serialize_analysis_msgpack(c: &mut Criterion) {
    use stacks_node_db_bench::db_current::clarity::CONTRACT_ANALYSIS;
    let analysis = ContractAnalysis::deserialize(CONTRACT_ANALYSIS).unwrap();

    c.bench_function("serialization/msgpack/analysis", |b| {
        b.iter(|| {
            let serialized = rmp_serde::to_vec(&analysis).unwrap();
            rmp_serde::from_slice::<ContractAnalysis>(&serialized).unwrap();
        });
    });
}