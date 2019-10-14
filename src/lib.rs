#[macro_use]
extern crate diesel;

table! {
    block (hash_id) {
        time -> BigInt,
        height -> Integer,
        extinct -> Bool,
        hash_id -> Binary,
        hash_rest -> Binary,
        prev_hash_id -> Binary,
        merkle_root -> Binary,
    }
}

table! {
    block_tx (block_hash_id) {
        block_hash_id -> Binary,
        tx_hash_id -> Binary,
    }
}

table! {
    input (output_tx_hash_id, output_tx_idx) {
        output_tx_idx -> Integer,
        has_witness -> Bool,
        output_tx_hash_id -> Binary,
        tx_hash_id -> Binary,
    }
}

table! {
    output (tx_hash_id, tx_idx) {
        value -> BigInt,
        tx_idx -> Integer,
        tx_hash_id -> Binary,
        address -> Nullable<Text>,
    }
}

table! {
    tx (hash_id) {
        mempool_ts -> Nullable<Timestamp>,
        fee -> BigInt,
        locktime -> BigInt,
        current_height -> Integer,
        weight -> Integer,
        coinbase -> Bool,
        hash_id -> Binary,
        hash_rest -> Binary,
    }
}

table! {
    tx_with_block (hash_id) {
        mempool_ts -> Nullable<Timestamp>,
        fee -> BigInt,
        locktime -> BigInt,
        current_height -> Integer,
        weight -> Integer,
        coinbase -> Bool,
        hash_id -> Binary,
        hash_rest -> Binary,
        block_hash_id -> Binary,
        block_hash_rest -> Binary,
        block_height -> Integer,
        block_prev_hash_id -> Binary,
        block_merkle_root -> Binary,
        block_extinct -> Bool,
        block_time -> BigInt,
        block_indexed_ts -> Timestamp,
        ts -> Timestamptz,
        indexed_ts -> Timestamp,
    }
}

table! {
    tx_in_mempool (hash_id) {
        mempool_ts -> Nullable<Timestamp>,
        fee -> BigInt,
        locktime -> BigInt,
        current_height -> Integer,
        weight -> Integer,
        coinbase -> Bool,
        hash_id -> Binary,
        hash_rest -> Binary,
    }
}

allow_tables_to_appear_in_same_query!(block, block_tx, input, output, tx,);
