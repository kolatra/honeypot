use std::{sync::Arc, net::SocketAddr};

use anyhow::{anyhow, bail};
use azalea::protocol::{
    connect::Connection,
    packets::{
        handshake::{ClientboundHandshakePacket, ServerboundHandshakePacket},
        status::{
            clientbound_pong_response_packet::ClientboundPongResponsePacket,
            ClientboundStatusPacket, ServerboundStatusPacket,
        },
        ConnectionProtocol, login::{ServerboundLoginPacket, ClientboundLoginPacket}
    },
};
use tokio::net::TcpStream;

use crate::{templates::*, webhook, GlobalData, Result};

type ServerHandshakeConn = Connection<ServerboundHandshakePacket, ClientboundHandshakePacket>;
type ServerStatusConn = Connection<ServerboundStatusPacket, ClientboundStatusPacket>;
type ServerLoginConn = Connection<ServerboundLoginPacket, ClientboundLoginPacket>;

pub async fn handle_conn(_state: Arc<GlobalData>, incoming: TcpStream) -> Result<()> {
    let peer = incoming.peer_addr()?;
    let mut conn: ServerHandshakeConn = Connection::wrap(incoming);

    // read the packet, if it's not a minecraft handshake, return early
    let Ok(ServerboundHandshakePacket::ClientIntention(handshake)) = conn.read().await else {
        webhook::log_connection(peer).await?;
        return Ok(());
    };

    println!("[*] handshake: {:?}", handshake);

    match handshake.intention {
        ConnectionProtocol::Status => entice(Connection::from(conn), peer, &handshake.hostname).await,
        ConnectionProtocol::Login => scare_away(Connection::from(conn), peer).await,
        _ => Err(anyhow!("[!] unexpected data")),
    }
}

async fn entice(mut conn: ServerStatusConn, peer: SocketAddr, target: &str) -> Result<()> {
    let ServerboundStatusPacket::StatusRequest(_) = conn.read().await? else {
        bail!("[!] expected status request")
    };

    webhook::log_mc_ping(peer, target).await?;
 
    let status = pick_for_me();
    conn.write(status.get()).await?;

    let ServerboundStatusPacket::PingRequest(ping_request) = conn.read().await? else {
        bail!("[!] expected ping request")
    };

    let ping_response = ClientboundPongResponsePacket {
        time: ping_request.time,
    };
    conn.write(ping_response.get()).await?;

    Ok(())
}

async fn scare_away(mut conn: ServerLoginConn, peer: SocketAddr) -> Result<()> {
    let ServerboundLoginPacket::Hello(_) = conn.read().await? else {
        bail!("[!] expected login start")
    };

    webhook::log_join(peer).await?;
    
    // TODO: send Login Success and Login Play before disconnecting

    let dc = disconnect("Internal server error, please try again later.");
    conn.write(dc.get()).await?;

    Ok(())
}
