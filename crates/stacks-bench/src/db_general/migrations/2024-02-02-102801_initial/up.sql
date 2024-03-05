CREATE TABLE block_hash_as_binary_1 (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    block_hash BINARY NOT NULL
);
CREATE INDEX ix_block_hash_as_binary_1_block_hash ON block_hash_as_binary_1 (block_hash);

CREATE TABLE block_hash_as_text_1 (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    block_hash TEXT NOT NULL
);
CREATE INDEX ix_block_hash_as_text_1_block_hash ON block_hash_as_text_1 (block_hash);

CREATE TABLE block_hash_as_binary_10 (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    block_hash_1 BINARY NOT NULL,
    block_hash_2 BINARY NOT NULL,
    block_hash_3 BINARY NOT NULL,
    block_hash_4 BINARY NOT NULL,
    block_hash_5 BINARY NOT NULL,
    block_hash_6 BINARY NOT NULL,
    block_hash_7 BINARY NOT NULL,
    block_hash_8 BINARY NOT NULL,
    block_hash_9 BINARY NOT NULL,
    block_hash_10 BINARY NOT NULL
);
CREATE INDEX ix_block_hash_as_binary_10_block_hash ON block_hash_as_binary_10 (block_hash_1);

CREATE TABLE block_hash_as_text_10 (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    block_hash_1 TEXT NOT NULL,
    block_hash_2 TEXT NOT NULL,
    block_hash_3 TEXT NOT NULL,
    block_hash_4 TEXT NOT NULL,
    block_hash_5 TEXT NOT NULL,
    block_hash_6 TEXT NOT NULL,
    block_hash_7 TEXT NOT NULL,
    block_hash_8 TEXT NOT NULL,
    block_hash_9 TEXT NOT NULL,
    block_hash_10 TEXT NOT NULL
);
CREATE INDEX ix_block_hash_as_text_10_block_hash ON block_hash_as_text_10 (block_hash_1);