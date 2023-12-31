use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::stats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Host {
    pub id: Uuid,
    pub ip_address: String,
    pub ping_count: i32,
    pub join_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::stats)]
pub struct NewEntry<'a> {
    pub id: Uuid,
    pub ip_address: &'a str,
    pub ping_count: i32,
    pub join_count: i32,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Player {
    pub uuid: Uuid,
    pub server_uuid: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::players)]
pub struct NewPlayer<'a> {
    pub uuid: Uuid,
    pub server_uuid: Uuid,
    pub name: &'a str,
}
