// @generated automatically by Diesel CLI.

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
