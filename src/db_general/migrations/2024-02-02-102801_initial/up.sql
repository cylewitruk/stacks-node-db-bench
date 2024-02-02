CREATE TABLE block_hash_as_binary (
    id INTEGER PRIMARY KEY,
    block_hash BINARY NOT NULL
);
CREATE INDEX ix_block_hash_as_binary_block_hash ON block_hash_as_binary (block_hash);

CREATE TABLE block_hash_as_text (
    id INTEGER PRIMARY KEY,
    block_hash TEXT NOT NULL
);
CREATE INDEX ix_block_hash_as_text_block_hash ON block_hash_as_text (block_hash);