--INSERT INTO metadata_store_type (id, name) VALUES (0, 'Data Map');
--INSERT INTO metadata_store_type (id, name) VALUES (1, 'Variable');
--INSERT INTO metadata_store_type (id, name) VALUES (2, 'Fungible Token');
--INSERT INTO metadata_store_type (id, name) VALUES (3, 'Circulating Supply');
--INSERT INTO metadata_store_type (id, name) VALUES (4, 'Non-Fungible Token');
----INSERT INTO metadata_store_type (id, name) VALUES (5, 'Data Map Metadata');
----INSERT INTO metadata_store_type (id, name) VALUES (6, 'Variable Metadata');
----INSERT INTO metadata_store_type (id, name) VALUES (7, 'Fungible Token Metadata');
----INSERT INTO metadata_store_type (id, name) VALUES (8, 'Non-Fungible Token Metadata');
----INSERT INTO metadata_store_type (id, name) VALUES (9, 'Contract');
--INSERT INTO metadata_store_type (id, name) VALUES (16, 'Simmed Block');
--INSERT INTO metadata_store_type (id, name) VALUES (17, 'Simmed Block Height');
--INSERT INTO metadata_store_type (id, name) VALUES (18, 'Nonce');
--INSERT INTO metadata_store_type (id, name) VALUES (19, 'STX Balance');
--INSERT INTO metadata_store_type (id, name) VALUES (20, 'PoX STX Lockup');
--INSERT INTO metadata_store_type (id, name) VALUES (21, 'PoX Unlock Height');

CREATE TABLE contract (
    contract_issuer TEXT NOT NULL,
    contract_name TEXT NOT NULL,
    block_hash BINARY NOT NULL,
    source_code BINARY NOT NULL,
    data_size INTEGER NOT NULL,
    contract_size INTEGER NOT NULL,
    ast BINARY NOT NULL,

    PRIMARY KEY (contract_issuer, contract_name, block_hash)
) WITHOUT ROWID;

CREATE TABLE data_map (
    contract_issuer TEXT NOT NULL,
    contract_name TEXT NOT NULL,
    block_hash BINARY NOT NULL,
    name TEXT NOT NULL,
    metadata BINARY NOT NULL,

    PRIMARY KEY (contract_issuer, contract_name, block_hash)
) WITHOUT ROWID;


--clr-meta::SPZ0RAC1EFTH949T4W2SYY6YBHJRMAF4ECT5A7DD.oracle-v1::vm-metadata::6::offsets-amount
--
--The above would be stored in the `variable` tabl as follows:
--  contract_issuer: SPZ0RAC1EFTH949T4W2SYY6YBHJRMAF4ECT5A7DD
--  contract_name: oracle-v1
--  block_hash: [from the `blockhash` column in `metadata_table`]
--  name: offsets-amount
--  metadata: [from the `value` column in `metadata_table`]
CREATE TABLE variable (
    contract_issuer TEXT NOT NULL,
    contract_name TEXT NOT NULL,
    block_hash BINARY NOT NULL,
    name TEXT NOT NULL,
    metadata BINARY NOT NULL,

    PRIMARY KEY (contract_issuer, contract_name, block_hash)
) WITHOUT ROWID;

CREATE TABLE fungible_token (
    contract_issuer TEXT NOT NULL,
    contract_name TEXT NOT NULL,
    block_hash BINARY NOT NULL,
    name TEXT NOT NULL,
    metadata,

    PRIMARY KEY (contract_issuer, contract_name, block_hash)
) WITHOUT ROWID;

CREATE TABLE non_fungible_token (
    contract_issuer TEXT NOT NULL,
    contract_name TEXT NOT NULL,
    block_hash BINARY NOT NULL,
    name TEXT NOT NULL,
    metadata,

    PRIMARY KEY (contract_issuer, contract_name, block_hash)
) WITHOUT ROWID;