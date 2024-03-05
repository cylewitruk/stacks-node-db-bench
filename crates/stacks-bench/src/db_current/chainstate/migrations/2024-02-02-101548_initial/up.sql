-- Your SQL goes here
CREATE TABLE marf_data (
   block_id INTEGER PRIMARY KEY, 
   block_hash TEXT UNIQUE NOT NULL,
   -- the trie itself.
   -- if not used, then set to a zero-byte entry.
   data BLOB NOT NULL,
   unconfirmed INTEGER NOT NULL
, external_offset INTEGER DEFAULT 0 NOT NULL, external_length INTEGER DEFAULT 0 NOT NULL);
CREATE TABLE mined_blocks (
   block_id INTEGER PRIMARY KEY, 
   block_hash TEXT UNIQUE NOT NULL,
   data BLOB NOT NULL
);
CREATE TABLE block_extension_locks (block_hash TEXT PRIMARY KEY);
CREATE TABLE schema_version (
    version INTEGER DEFAULT 1 NOT NULL
);
CREATE TABLE migrated_version (
    version INTEGER DEFAULT 1 NOT NULL
);
CREATE INDEX block_hash_marf_data ON marf_data(block_hash);
CREATE INDEX unconfirmed_marf_data ON marf_data(unconfirmed);
CREATE INDEX block_hash_mined_blocks ON mined_blocks(block_hash);
CREATE INDEX index_external_offset ON marf_data(external_offset);
CREATE TABLE block_headers(
        version INTEGER NOT NULL,
        total_burn TEXT NOT NULL,       -- converted to/from u64
        total_work TEXT NOT NULL,       -- converted to/from u64
        proof TEXT NOT NULL,
        parent_block TEXT NOT NULL,             -- hash of parent Stacks block
        parent_microblock TEXT NOT NULL,
        parent_microblock_sequence INTEGER NOT NULL,
        tx_merkle_root TEXT NOT NULL,
        state_index_root TEXT NOT NULL,
        microblock_pubkey_hash TEXT NOT NULL,
        
        block_hash TEXT NOT NULL,                   -- NOTE: this is *not* unique, since two burn chain forks can commit to the same Stacks block.
        index_block_hash TEXT UNIQUE NOT NULL,      -- NOTE: this is the hash of the block hash and consensus hash of the burn block that selected it, 
                                                    -- and is guaranteed to be globally unique (across all Stacks forks and across all PoX forks).
                                                    -- index_block_hash is the block hash fed into the MARF index.

        -- internal use only
        block_height INTEGER NOT NULL,
        index_root TEXT NOT NULL,                    -- root hash of the internal, not-consensus-critical MARF that allows us to track chainstate /fork metadata
        consensus_hash TEXT UNIQUE NOT NULL,         -- all consensus hashes are guaranteed to be unique
        burn_header_hash TEXT NOT NULL,              -- burn header hash corresponding to the consensus hash (NOT guaranteed to be unique, since we can have 2+ blocks per burn block if there's a PoX fork)
        burn_header_height INT NOT NULL,             -- height of the burnchain block header that generated this consensus hash
        burn_header_timestamp INT NOT NULL,          -- timestamp from burnchain block header that generated this consensus hash
        parent_block_id TEXT NOT NULL,               -- NOTE: this is the parent index_block_hash

        cost TEXT NOT NULL,
        block_size TEXT NOT NULL,       -- converted to/from u64
        affirmation_weight INTEGER NOT NULL,

        PRIMARY KEY(consensus_hash,block_hash)
    );
CREATE TABLE payments(
        address TEXT NOT NULL,              -- miner that produced this block and microblock stream
        block_hash TEXT NOT NULL,
        consensus_hash TEXT NOT NULL,
        parent_block_hash TEXT NOT NULL,
        parent_consensus_hash TEXT NOT NULL,
        coinbase TEXT NOT NULL,             -- encodes u128
        tx_fees_anchored TEXT NOT NULL,     -- encodes u128
        tx_fees_streamed TEXT NOT NULL,     -- encodes u128
        stx_burns TEXT NOT NULL,            -- encodes u128
        burnchain_commit_burn INT NOT NULL,
        burnchain_sortition_burn INT NOT NULL,
        miner INT NOT NULL,
        
        -- internal use
        stacks_block_height INTEGER NOT NULL,
        index_block_hash TEXT NOT NULL,     -- NOTE: can't enforce UNIQUE here, because there will be multiple entries per block
        vtxindex INT NOT NULL               -- user burn support vtxindex
    , recipient TEXT);
CREATE TABLE user_supporters(
        address TEXT NOT NULL,
        support_burn INT NOT NULL,
        block_hash TEXT NOT NULL,
        consensus_hash TEXT NOT NULL,

        PRIMARY KEY(address,block_hash,consensus_hash)
    );
CREATE TABLE db_config(
        version TEXT NOT NULL,
        mainnet INTEGER NOT NULL,
        chain_id INTEGER NOT NULL
    );
CREATE TABLE staging_microblocks(anchored_block_hash TEXT NOT NULL,     -- this is the hash of the parent anchored block
                                     consensus_hash TEXT NOT NULL,          -- this is the hash of the burn chain block that holds the parent anchored block's block-commit
                                     index_block_hash TEXT NOT NULL,        -- this is the anchored block's index hash
                                     microblock_hash TEXT NOT NULL,
                                     parent_hash TEXT NOT NULL,             -- previous microblock
                                     index_microblock_hash TEXT NOT NULL,   -- this is the hash of consensus_hash and microblock_hash
                                     sequence INT NOT NULL,
                                     processed INT NOT NULL,
                                     orphaned INT NOT NULL,
                                     PRIMARY KEY(anchored_block_hash,consensus_hash,microblock_hash)
    );
CREATE TABLE staging_microblocks_data(block_hash TEXT NOT NULL,
                                          block_data BLOB NOT NULL,
                                          PRIMARY KEY(block_hash)
    );
CREATE TABLE invalidated_microblocks_data(block_hash TEXT NOT NULL,
                                              block_data BLOB NOT NULL,
                                              PRIMARY KEY(block_hash)
    );
CREATE TABLE staging_blocks(anchored_block_hash TEXT NOT NULL,
                                parent_anchored_block_hash TEXT NOT NULL,
                                consensus_hash TEXT NOT NULL,
                                -- parent_consensus_hash is the consensus hash of the sortition that chose the parent Stacks block.
                                parent_consensus_hash TEXT NOT NULL,
                                parent_microblock_hash TEXT NOT NULL,
                                parent_microblock_seq INT NOT NULL,
                                microblock_pubkey_hash TEXT NOT NULL,
                                height INT NOT NULL,
                                attachable INT NOT NULL,            -- set to 1 if this block's parent is processed; 0 if not
                                orphaned INT NOT NULL,              -- set to 1 if this block can never be attached
                                processed INT NOT NULL,
                                commit_burn INT NOT NULL,
                                sortition_burn INT NOT NULL,
                                index_block_hash TEXT NOT NULL,           -- used internally; hash of consensus hash and anchored_block_hash
                                download_time INT NOT NULL,               -- how long the block was in-flight
                                arrival_time INT NOT NULL,                -- when this block was stored
                                processed_time INT NOT NULL,              -- when this block was processed
                                PRIMARY KEY(anchored_block_hash,consensus_hash)
    );
CREATE TABLE staging_user_burn_support(anchored_block_hash TEXT NOT NULL,
                                           consensus_hash TEXT NOT NULL,
                                           address TEXT NOT NULL,
                                           burn_amount INT NOT NULL,
                                           vtxindex INT NOT NULL
    );
CREATE TABLE transactions(
        id INTEGER PRIMARY KEY,
        txid TEXT NOT NULL,
        index_block_hash TEXT NOT NULL,
        tx_hex TEXT NOT NULL,
        result TEXT NOT NULL,
        UNIQUE (txid,index_block_hash)
    );
CREATE TABLE epoch_transitions(
        block_id TEXT PRIMARY KEY
    );
CREATE TABLE matured_rewards(
        address TEXT NOT NULL,      -- address of the miner who produced the block
        recipient TEXT,             -- who received the reward (if different from the miner)
        vtxindex INTEGER NOT NULL,  -- will be 0 if this is the miner, >0 if this is a user burn support
        coinbase TEXT NOT NULL,
        tx_fees_anchored TEXT NOT NULL,
        tx_fees_streamed_confirmed TEXT NOT NULL,
        tx_fees_streamed_produced TEXT NOT NULL,

        -- fork identifier 
        child_index_block_hash TEXT NOT NULL,
        parent_index_block_hash TEXT NOT NULL,

        -- there are two rewards records per (parent,child) pair. One will have a non-zero coinbase; the other will have a 0 coinbase.
        PRIMARY KEY(parent_index_block_hash,child_index_block_hash,coinbase)
    );
CREATE INDEX index_matured_rewards_by_vtxindex ON matured_rewards(parent_index_block_hash,child_index_block_hash,vtxindex);
CREATE INDEX index_parent_block_id_by_block_id ON block_headers(index_block_hash,parent_block_id);
CREATE TABLE burnchain_txids(
        index_block_hash TEXT PRIMARY KEY,
        -- this is a JSON-encoded list of txids
        txids TEXT NOT NULL
    );
CREATE INDEX index_block_hash_to_primary_key ON block_headers(index_block_hash,consensus_hash,block_hash);
CREATE INDEX block_headers_hash_index ON block_headers(block_hash,block_height);
CREATE INDEX block_index_hash_index ON block_headers(index_block_hash,consensus_hash,block_hash);
CREATE INDEX block_headers_burn_header_height ON block_headers(burn_header_height);
CREATE INDEX index_payments_block_hash_consensus_hash_vtxindex ON payments(block_hash,consensus_hash,vtxindex ASC);
CREATE INDEX index_payments_index_block_hash_vtxindex ON payments(index_block_hash,vtxindex ASC);
CREATE INDEX staging_microblocks_processed ON staging_microblocks(processed);
CREATE INDEX staging_microblocks_orphaned ON staging_microblocks(orphaned);
CREATE INDEX staging_microblocks_index_hash ON staging_microblocks(index_block_hash);
CREATE INDEX staging_microblocks_index_hash_processed ON staging_microblocks(index_block_hash,processed);
CREATE INDEX staging_microblocks_index_hash_orphaned ON staging_microblocks(index_block_hash,orphaned);
CREATE INDEX staging_microblocks_microblock_hash ON staging_microblocks(microblock_hash);
CREATE INDEX processed_stacks_blocks ON staging_blocks(processed,anchored_block_hash,consensus_hash);
CREATE INDEX orphaned_stacks_blocks ON staging_blocks(orphaned,anchored_block_hash,consensus_hash);
CREATE INDEX parent_blocks ON staging_blocks(parent_anchored_block_hash);
CREATE INDEX parent_consensus_hashes ON staging_blocks(parent_consensus_hash);
CREATE INDEX index_block_hashes ON staging_blocks(index_block_hash);
CREATE INDEX height_stacks_blocks ON staging_blocks(height);
CREATE INDEX index_staging_user_burn_support ON staging_user_burn_support(anchored_block_hash,consensus_hash);
CREATE INDEX txid_tx_index ON transactions(txid);
CREATE INDEX index_block_hash_tx_index ON transactions(index_block_hash);
CREATE INDEX index_block_header_by_affirmation_weight ON block_headers(affirmation_weight);
CREATE INDEX index_block_header_by_height_and_affirmation_weight ON block_headers(block_height,affirmation_weight);
CREATE INDEX index_parent_block_id ON block_headers(parent_block_id);
/* bytecode(addr,opcode,p1,p2,p3,p4,p5,comment,subprog) */;
/* completion(candidate) */;
/* dbstat(name,path,pageno,pagetype,ncell,payload,unused,mx_payload,pgoffset,pgsize) */;
/* fsdir(name,mode,mtime,data) */;
/* fts3tokenize(input,token,start,"end",position) */;
/* generate_series(value) */;
/* json_each("key",value,type,atom,id,parent,fullkey,path) */;
/* json_tree("key",value,type,atom,id,parent,fullkey,path) */;
/* pragma_database_list(seq,name,file) */;
/* pragma_module_list(name) */;
/* sqlite_dbdata(pgno,cell,field,value) */;
/* sqlite_dbpage(pgno,data) */;
/* sqlite_dbptr(pgno,child) */;
/* sqlite_stmt(sql,ncol,ro,busy,nscan,nsort,naidx,nstep,reprep,run,mem) */;
/* tables_used(type,schema,name,wr,subprog) */;
/* zipfile(name,mode,mtime,sz,rawdata,data,method) */;