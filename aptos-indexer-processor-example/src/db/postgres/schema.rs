// @generated automatically by Diesel CLI.

diesel::table! {
    backfill_processor_status (backfill_alias) {
        #[max_length = 50]
        backfill_alias -> Varchar,
        #[max_length = 50]
        backfill_status -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
        last_transaction_timestamp -> Nullable<Timestamp>,
        backfill_start_version -> Int8,
        backfill_end_version -> Int8,
    }
}

diesel::table! {
    profiles (transaction_version, account_address) {
        #[max_length = 66]
        account_address -> Varchar,
        transaction_version -> Int8,
        transaction_block_height -> Int8,
        #[max_length = 300]
        name -> Varchar,
        #[max_length = 300]
        avatar_url -> Varchar,
        inserted_at -> Timestamp,
    }
}

diesel::table! {
    ledger_infos (chain_id) {
        chain_id -> Int8,
    }
}

diesel::table! {
    processor_status (processor) {
        #[max_length = 50]
        processor -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
        last_transaction_timestamp -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    backfill_processor_status,
    profiles,
    ledger_infos,
    processor_status,
);
