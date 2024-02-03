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
    block_hash_as_binary_1 (id) {
        id -> Integer,
        block_hash -> Binary,
    }
}

table! {
    block_hash_as_text_1 (id) {
        id -> Integer,
        block_hash -> Text,
    }
}

table! {
    block_hash_as_binary_10 (id) {
        id -> Integer,
        block_hash_1 -> Binary,
        block_hash_2 -> Binary,
        block_hash_3 -> Binary,
        block_hash_4 -> Binary,
        block_hash_5 -> Binary,
        block_hash_6 -> Binary,
        block_hash_7 -> Binary,
        block_hash_8 -> Binary,
        block_hash_9 -> Binary,
        block_hash_10 -> Binary,
    }
}

table! {
    block_hash_as_text_10 (id) {
        id -> Integer,
        block_hash_1 -> Text,
        block_hash_2 -> Text,
        block_hash_3 -> Text,
        block_hash_4 -> Text,
        block_hash_5 -> Text,
        block_hash_6 -> Text,
        block_hash_7 -> Text,
        block_hash_8 -> Text,
        block_hash_9 -> Text,
        block_hash_10 -> Text,
    }
}