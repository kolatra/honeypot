use std::sync::Arc;

use anyhow::anyhow;
use azalea::protocol::{
    connect::Connection,
    packets::{
        game::{ClientboundGamePacket, ServerboundGamePacket},
        handshake::{ClientboundHandshakePacket, ServerboundHandshakePacket},
        status::{
            clientbound_pong_response_packet::ClientboundPongResponsePacket,
            ClientboundStatusPacket, ServerboundStatusPacket,
        },
        ConnectionProtocol,
    },
};
use tokio::net::TcpStream;

use crate::{templates::*, webhook, GlobalData, Result};

type ServerHandshakeConn = Connection<ServerboundHandshakePacket, ClientboundHandshakePacket>;
type ServerStatusConn = Connection<ServerboundStatusPacket, ClientboundStatusPacket>;

#[allow(unused)]
type ServerGameConn = Connection<ServerboundGamePacket, ClientboundGamePacket>;

pub async fn scare_away(_state: Arc<GlobalData>, incoming: TcpStream) -> Result<()> {
    let incoming_addr = incoming.peer_addr()?;
    let mut conn: ServerHandshakeConn = Connection::wrap(incoming);

    // read the packet, if it's not a minecraft handshake, return early
    let Ok(ServerboundHandshakePacket::ClientIntention(handshake)) = conn.read().await else {
        webhook::log_connection(incoming_addr).await?;
        return Ok(());
    };

    println!("[*] handshake: {:?}", handshake);
    webhook::log_mc_ping(incoming_addr, &handshake.hostname).await?;

    match handshake.intention {
        ConnectionProtocol::Status => handle_status(Connection::from(conn)).await,
        ConnectionProtocol::Login => webhook::log_join(incoming_addr).await,
        _ => Err(anyhow!("[!] unexpected data")),
    }
}

async fn handle_status(mut conn: ServerStatusConn) -> Result<()> {
    let _ = match conn.read().await? {
        ServerboundStatusPacket::StatusRequest(request) => request,
        _ => return Err(anyhow!("[!] expected status request")),
    };

    let status = pick_for_me();
    conn.write(status.get()).await?;

    let ping_request = match conn.read().await? {
        ServerboundStatusPacket::PingRequest(ping_request) => ping_request,
        _ => return Err(anyhow!("[!] expected ping request")),
    };

    let ping_response = ClientboundPongResponsePacket {
        time: ping_request.time,
    };

    conn.write(ping_response.get()).await?;

    Ok(())
}
