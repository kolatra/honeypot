use std::env::var;

use diesel::{PgConnection, Connection, SelectableHelper, RunQueryDsl, result::Error as DbError, ExpressionMethods};

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

pub async fn new_entry(
    conn: &mut PgConnection, 
    addr: &str, 
    update_type: Update
) -> anyhow::Result<Host, DbError> {
    diesel::insert_into(stats)
        .values(&NewEntry {
            ip_address: addr,
            ping_count: 0,
            join_count: 0,
        })
        .on_conflict(ip_address)
        .do_update()
        .set((
            // O_O
            ping_count.eq(ping_count + ( if update_type == Update::Ping { 1 } else { 0 } )),
            join_count.eq(join_count + ( if update_type == Update::Join { 1 } else { 0 } )),
        ))
        .returning(Host::as_returning())
        .get_result(conn)
}

async fn unfinished_stuff() {
    use diesel::QueryDsl;
    use crate::schema::stats::dsl::*;

    let res = connect().await;
    dbg!(&res.is_ok());

    let mut conn = res.unwrap();

    let res = diesel::insert_into(stats)
        .values(&NewEntry {
            ip_address: "127.0.0.1",
            ping_count: 0,
            join_count: 0,
        })
        .returning(Host::as_returning())
        .get_result(&mut conn);

    dbg!(res.is_ok());

    let res = stats
        .filter(ip_address.eq("127.0.0.1"))
        .first::<crate::models::Host>(&mut conn);
    
    dbg!(res.is_ok());

    todo!()
}
