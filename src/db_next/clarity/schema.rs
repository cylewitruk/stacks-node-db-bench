use diesel::prelude::*;

table! {
    contract (contract_issuer, contract_name, block_hash) {
        contract_issuer -> Text,
        contract_name -> Text,
        block_hash -> Binary,
        source_code -> Binary,
        data_size -> Integer,
        contract_size -> Integer,
        ast -> Binary,
        ast_size -> Integer,
    }
}