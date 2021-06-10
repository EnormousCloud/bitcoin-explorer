-- `longest_chain` is the storage of the chain mapping 
-- of the block height to the block hash
-- the biggest height might be unconfirmed transactions
drop table if exists longest_chain;
create unlogged table longest_chain(
    blockheight int,    -- height of the block
    blockhash   bytea,  -- which block is trusted to be at that height
    PRIMARY KEY (blockheight)
);
create index idx_chain_hash on longest_chain (blockhash);

-- `final_blocks` table is the storage of blocks
-- 
-- it doesn't contain full block information, just the necessary minimum 
-- to be displayed in the list of blocks
-- all records in this table passed consensus. and the table should be never rewritten
drop table if exists final_blocks;
create table final_blocks (
    blockhash      bytea,  -- block hash
    blockheight    integer,    -- block height
    tm             timestamptz, -- time of this block (still height is the dominant)
    avgfee         bigint,
    avgfeerate     bigint,
    avgtxsize      bigint,
    ins            bigint,
    maxfee         bigint,
    maxfeerate     bigint,
    maxtxsize      bigint,
    medianfee      bigint,
    mediantxsize   bigint,
    minfee         bigint,
    minfeerate     bigint,
    mintxsize      bigint,
    outs           bigint,
    subsidy        bigint,
    swtotal_size   bigint,
    swtotal_weight bigint,
    swtxs          bigint,
    total_out      bigint,
    total_size     bigint,
    total_weight   bigint,
    totalfee       bigint,
    txs            bigint,
    utxo_increase  bigint,
    utxo_size_inc  bigint,
    primary key (blockhash)
);

create index idx_final_blocks_height on final_blocks (blockheight);

-- same as `final_blocks`, but there is no full consensus on the blocks inside this table
-- this is why it is in-memory and might disappear with restart
--
-- once block is considered final, it is gone from this database
drop table if exists temp_blocks;
create unlogged table temp_blocks (
    blockhash   bytea,  -- block hash
    blockheight bigint,    -- block height
    tm          timestamptz, -- time of this block (still height is the dominant)
    avgfee         bigint,
    avgfeerate     bigint,
    avgtxsize      bigint,
    height         integer,
    ins            bigint,
    maxfee         bigint,
    maxfeerate     bigint,
    maxtxsize      bigint,
    medianfee      bigint,
    mediantxsize   bigint,
    minfee         bigint,
    minfeerate     bigint,
    mintxsize      bigint,
    outs           bigint,
    subsidy        bigint,
    swtotal_size   bigint,
    swtotal_weight bigint,
    swtxs          bigint,
    total_out      bigint,
    total_size     bigint,
    total_weight   bigint,
    totalfee       bigint,
    txs            bigint,
    utxo_increase  bigint,
    utxo_size_inc  bigint,
    primary key (blockhash)
);

-- 
drop table if exists final_tx;
create table final_tx (
    txhash      bytea,  -- transaction hash
    txindex     int,    -- transaction index in this block
    blockhash   bytea,  -- block hash
    blockheight int,    -- block height
    sent_btc    bigint, -- total amount of BTC transferred in this transaction
    fee_btc     bigint, -- fee for this transaction
    primary key (txhash)
);

-- `tx` are recent transactions in memory buffer that are not considered final yet
drop table if exists tx;
create unlogged table tx (
    txhash      bytea,     -- transaction hash
    txindex     int,    -- transaction index in this block
    blockhash   bytea,  -- block hash
    blockheight int,    -- block height
    sent_btc    bigint, -- total amount of BTC transferred in this transaction
    fee_btc     bigint, -- fee for this transaction
    primary key (txhash)
);

-- `final_addr` are transactions groupped by address
drop table if exists final_addr;
create table final_addr (
    addr         bytea,    -- address of the wallet
    blockheight  int,      -- block height
    txindex      int,      -- transaction index in this block
    sent_btc     bigint,   -- total amount of BTC transferred in this transaction. if negative, this is fee deduction
    primary key (addr, blockheight, txindex)
);
create index idx_final_addr_addr on final_addr (addr);
create index idx_final_addr_blockheight on final_addr (blockheight, txindex);

-- `addr` are address transaction history
drop table if exists unconfirmed_addr;
create unlogged table unconfirmed_addr (
    addr         bytea,    -- address of the wallet
    txhash       bytea,    -- transaction hash
    blockhash    bytea,    -- block hash
    blockheight  int,      -- block height
    txindex      int,      -- transaction index in this block
    sent_btc     bigint,   -- total amount of BTC transferred in this transaction
    fee_btc      bigint,   -- fee paid for this transaction (if that was a sender record)
    primary key (addr, txhash, txindex)
);
create index idx_addr_addr on unconfirmed_addr (addr);
create index idx_addr_blockhash on unconfirmed_addr (blockhash, txindex);
create index idx_addr_txhash on unconfirmed_addr (txhash);
