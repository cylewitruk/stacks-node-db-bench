-- This file should undo anything in `up.sql`
DROP TABLE marf_data;
DROP TABLE mined_blocks;
DROP TABLE block_extension_locks;
DROP TABLE schema_version;
DROP TABLE migrated_version;
DROP TABLE block_headers;
DROP TABLE payments;
DROP TABLE user_supporters;
DROP TABLE db_config;
DROP TABLE staging_microblocks;
DROP TABLE staging_microblocks_data;
DROP TABLE invalidated_microblocks_data;
DROP TABLE staging_blocks;
DROP TABLE staging_user_burn_support;
DROP TABLE transactions;
DROP TABLE epoch_transitions;
DROP TABLE matured_rewards;
DROP TABLE burnchain_txids;
DROP TABLE __fork_storage;