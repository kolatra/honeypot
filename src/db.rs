use diesel::{
    result::Error as DbError, Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use uuid::Uuid;

use crate::{
    models::Host,
    models::{NewEntry, NewPlayer},
    schema::stats::dsl::*,
};

pub fn connect() -> anyhow::Result<PgConnection> {
    let url = &crate::CONFIG.db_url;
    let conn = PgConnection::establish(url)?;
    Ok(conn)
}

#[derive(PartialEq, Eq)]
pub enum Update {
    Ping,
    Join,
}

#[allow(unused)]
pub fn add_or_update(
    conn: &mut PgConnection,
    addr: &str,
    update_type: Update,
) -> anyhow::Result<Host, DbError> {
    let pc = i32::from(update_type == Update::Ping);
    let jc = i32::from(update_type == Update::Join);

    let query = stats.filter(ip_address.eq(addr)).first::<Host>(conn);

    match query {
        Ok(addr) => diesel::update(stats)
            .filter(ip_address.eq(addr.ip_address))
            .set((
                ping_count.eq(addr.ping_count + pc),
                join_count.eq(addr.join_count + jc),
                updated_at.eq(chrono::Local::now().naive_local()),
            ))
            .returning(Host::as_returning())
            .get_result(conn),

        Err(e) => {
            if e != DbError::NotFound {
                return Err(e);
            }

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

pub fn add_player(
    conn: &mut PgConnection,
    addr: &str,
    name: &str,
    uuid: Uuid,
) -> anyhow::Result<()> {
    let backup = NewEntry {
        id: uuid::Uuid::new_v4(),
        ip_address: addr,
        ping_count: 0,
        join_count: 1,
    };

    let server = stats
        .filter(ip_address.eq(addr))
        .first::<Host>(conn)
        .unwrap_or_else(|_| {
            diesel::insert_into(stats)
                .values(&backup)
                .returning(Host::as_returning())
                .get_result(conn)
                .unwrap()
        });

    let player = NewPlayer {
        uuid,
        server_uuid: server.id,
        name,
    };

    diesel::insert_into(crate::schema::players::table)
        .values(&player)
        .on_conflict(crate::schema::players::uuid)
        .do_update()
        .set(crate::schema::players::updated_at.eq(chrono::Local::now().naive_local()))
        .execute(conn)?;

    Ok(())
}
