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
CREATE TABLE data_table
                      (key TEXT PRIMARY KEY, value TEXT);
CREATE TABLE metadata_table
                      (key TEXT NOT NULL, blockhash TEXT, value TEXT,
                       UNIQUE (key, blockhash));
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