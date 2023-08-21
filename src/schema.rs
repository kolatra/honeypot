// @generated automatically by Diesel CLI.

diesel::table! {
    players (uuid) {
        uuid -> Uuid,
        server_uuid -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    stats (id) {
        id -> Uuid,
        #[max_length = 255]
        ip_address -> Varchar,
        ping_count -> Int4,
        join_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(players -> stats (server_uuid));

diesel::allow_tables_to_appear_in_same_query!(players, stats,);
