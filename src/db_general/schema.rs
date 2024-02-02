/*
CREATE TABLE block_hash_as_binary (
    id INTEGER PRIMARY KEY,
    block_hash BLOB NOT NULL
);

CREATE TABLE block_hash_as_hex (
    id INTEGER PRIMARY KEY,
    block_hash TEXT NOT NULL
); */

use diesel::table;

table! {
    block_hash_as_binary (id) {
        id -> Integer,
        block_hash -> Binary,
    }
}

table! {
    block_hash_as_text (id) {
        id -> Integer,
        block_hash -> Text,
    }
}