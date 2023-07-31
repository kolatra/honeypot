use std::sync::Arc;

use anyhow::anyhow;
use azalea::protocol::{
    connect::Connection,
    packets::{
        handshake::{ClientboundHandshakePacket, ServerboundHandshakePacket},
        status::{
            clientbound_pong_response_packet::ClientboundPongResponsePacket,
            clientbound_status_response_packet::{
                ClientboundStatusResponsePacket, Players, SamplePlayer, Version,
            },
            ClientboundStatusPacket, ServerboundStatusPacket,
        },
        ConnectionProtocol,
    },
};
use azalea_chat::{text_component::TextComponent, FormattedText};
use tokio::net::TcpStream;

use crate::{GlobalData, Result};

type ServerHandshakeConn = Connection<ServerboundHandshakePacket, ClientboundHandshakePacket>;
type ServerStatusConn = Connection<ServerboundStatusPacket, ClientboundStatusPacket>;

pub async fn scare_away(_state: Arc<GlobalData>, incoming: TcpStream) -> Result<()> {
    let mut conn: ServerHandshakeConn = Connection::wrap(incoming);
    let ServerboundHandshakePacket::ClientIntention(handshake) = conn.read().await?;
    println!("[*] handshake: {:?}", handshake);

    match handshake.intention {
        ConnectionProtocol::Status => handle_status(Connection::from(conn)).await?,
        ConnectionProtocol::Login => {
            println!("[*] login connection, ignoring for now");
            return Ok(());
        }
        _ => return Err(anyhow!("[!] unexpected data")),
    }

    Ok(())
}

async fn handle_status(mut conn: ServerStatusConn) -> Result<()> {
    let _ = match conn.read().await? {
        ServerboundStatusPacket::StatusRequest(request) => request,
        _ => return Err(anyhow!("[!] expected status request")),
    };

    let status = Responses::many_anonymous_players(10);
    conn.write(status.get()).await?;

    let ping_request = match conn.read().await? {
        ServerboundStatusPacket::PingRequest(ping_request) => ping_request,
        _ => return Err(anyhow!("[!] expected ping request")),
    };

    let ping_response = ClientboundPongResponsePacket {
        time: ping_request.time, // make this configurable for fun?
    };

    conn.write(ping_response.get()).await?;

    Ok(())
}

struct Responses;
#[allow(unused)]
impl Responses {
    fn base(motd: Option<&str>, players: Option<Players>) -> ClientboundStatusResponsePacket {
        let s = motd.unwrap_or("A Minecraft Server");
        let text = TextComponent::new(s.to_string());

        ClientboundStatusResponsePacket {
            description: FormattedText::Text(text),
            favicon: None,
            players: players.unwrap_or(Players {
                max: 20,
                online: 0,
                sample: vec![],
            }),
            version: Version {
                name: "1.20.1".to_string(),
                protocol: 763,
            },
            enforces_secure_chat: Some(false),
        }
    }

    fn base_response() -> ClientboundStatusResponsePacket {
        Self::base(None, None)
    }

    fn dream_response() -> ClientboundStatusResponsePacket {
        let players = Players {
            max: 20,
            online: 3,
            sample: vec![
                SamplePlayer {
                    id: "ec70bcaf-702f-4bb8-b48d-276fa52a780c".to_string(),
                    name: "Dream".to_string(),
                },
                SamplePlayer {
                    id: "bd3dd5a4-0438-4699-b2fd-36f518154b41".to_string(),
                    name: "GeorgeNotFound".to_string(),
                },
                SamplePlayer {
                    id: "c66f7c8a-ed0c-4469-90b0-421d8ff7ca49".to_string(),
                    name: "Sapnap".to_string(),
                },
            ],
        };

        Self::base(Some("Dream Recording Server"), Some(players))
    }

    fn many_anonymous_players(online: i32) -> ClientboundStatusResponsePacket {
        let players = Players {
            max: 20,
            online,
            sample: (0..=online)
                .map(|i| SamplePlayer {
                    id: "00000000-0000-0000-0000-000000000000".to_string(),
                    name: "Anonymous Player".to_string(),
                })
                .collect(),
        };

        Self::base(None, Some(players))
    }
}
