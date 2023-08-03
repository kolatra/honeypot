use std::sync::Arc;

use anyhow::anyhow;
use azalea::protocol::{
    connect::Connection,
    packets::{
        handshake::{ClientboundHandshakePacket, ServerboundHandshakePacket},
        status::{
            clientbound_pong_response_packet::ClientboundPongResponsePacket,
            ClientboundStatusPacket, ServerboundStatusPacket,
        },
        ConnectionProtocol, login::{ServerboundLoginPacket, ClientboundLoginPacket},
    },
};
use tokio::net::TcpStream;

use crate::{templates::*, webhook, GlobalData, Result};

type ServerHandshakeConn = Connection<ServerboundHandshakePacket, ClientboundHandshakePacket>;
type ServerStatusConn = Connection<ServerboundStatusPacket, ClientboundStatusPacket>;
type ServerLoginConn = Connection<ServerboundLoginPacket, ClientboundLoginPacket>;

pub async fn handle_conn(_state: Arc<GlobalData>, incoming: TcpStream) -> Result<()> {
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
        ConnectionProtocol::Status => entice(Connection::from(conn)).await,
        ConnectionProtocol::Login => {
            webhook::log_join(incoming_addr).await?;
            scare_away(Connection::from(conn)).await
        },
        _ => Err(anyhow!("[!] unexpected data")),
    }
}

async fn entice(mut conn: ServerStatusConn) -> Result<()> {
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

async fn scare_away(mut conn: ServerLoginConn) -> Result<()> {
    let _ = match conn.read().await? {
        ServerboundLoginPacket::Hello(hello) => hello,
        _ => return Err(anyhow!("[!] expected login start")),
    };

    let dc = disconnect("You found a honeypot server designed to find scanners, please remove this IP from your list.");
    conn.write(dc.get()).await?;

    Ok(())
}
