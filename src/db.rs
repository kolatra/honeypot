use std::env::var;

use diesel::{PgConnection, Connection, SelectableHelper, RunQueryDsl, result::Error as DbError, ExpressionMethods, QueryDsl};

use crate::{models::Host, schema::stats::dsl::*, models::NewEntry};

pub async fn connect() -> anyhow::Result<PgConnection> {
    let url = var("DATABASE_URL")?;
    Ok(PgConnection::establish(&url)?)
}

#[derive(PartialEq)]
pub enum Update {
    Ping,
    Join,
}

#[allow(unused)]
pub async fn add_or_update(
    conn: &mut PgConnection, 
    addr: &str, 
    update_type: Update
) -> anyhow::Result<Host, DbError> {
    let pc = if update_type == Update::Ping { 1 } else { 0 }; 
    let jc = if update_type == Update::Join { 1 } else { 0 };

    let query = stats
        .filter(ip_address.eq(addr))
        .first::<Host>(conn);

    let existing_addr = match query {
        Ok(a) => Some(a),
        Err(e) => {
            if e == DbError::NotFound {
                None
            } else {
                return Err(e);
            }
        }
    };

    match existing_addr {
        Some(addr) => {
            diesel::update(stats)
                .filter(ip_address.eq(addr.ip_address))
                .set((
                    ping_count.eq(addr.ping_count + pc),
                    join_count.eq(addr.join_count + jc),
                    updated_at.eq(chrono::Local::now().naive_local()),
                ))
                .returning(Host::as_returning())
                .get_result(conn)
        },

        None => {
            let new_entry = NewEntry {
                id: uuid::Uuid::new_v4(),
                ip_address: addr,
                ping_count: pc,
                join_count: jc,
            };

            diesel::insert_into(stats)
                .values(&new_entry)
                .returning(Host::as_returning())
                .get_result(conn)
        }
    }
}
