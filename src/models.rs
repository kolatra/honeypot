use diesel::{Queryable, Selectable, Insertable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::stats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Host {
    pub id: i32,
    pub ip_address: String,
    pub ping_count: i32,
    pub join_count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::stats)]
pub struct NewEntry<'a> {
    pub ip_address: &'a str,
    pub ping_count: i32,
    pub join_count: i32,
}
