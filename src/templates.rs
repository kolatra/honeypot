use azalea::protocol::packets::login::clientbound_login_disconnect_packet::ClientboundLoginDisconnectPacket;
use azalea::protocol::packets::status::clientbound_status_response_packet::ClientboundStatusResponsePacket as Status;
use azalea::protocol::packets::status::clientbound_status_response_packet::{
    Players, SamplePlayer, Version,
};
use azalea_chat::{text_component::TextComponent, FormattedText};

pub fn disconnect(reason: &str) -> ClientboundLoginDisconnectPacket {
    ClientboundLoginDisconnectPacket {
        reason: FormattedText::Text(TextComponent::new(reason.to_string())),
    }
}

pub fn status_response(s: Option<&str>, players: Option<Players>) -> Status {
    let motd = s.unwrap_or("A Minecraft Server");
    let text = TextComponent::new(motd.to_string());

    Status {
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

pub fn pick_for_me() -> Status {
    let choices = vec![base(), anonymous(fastrand::i32(0..=20)), dream()];

    choices[fastrand::usize(..choices.len())].clone()
}

pub fn base() -> Status {
    status_response(None, None)
}

pub fn anonymous(online: i32) -> Status {
    let players = Players {
        max: 20,
        online,
        sample: (0..=online)
            .map(|_| SamplePlayer {
                id: "00000000-0000-0000-0000-000000000000".to_string(),
                name: "Anonymous Player".to_string(),
            })
            .collect(),
    };

    status_response(None, Some(players))
}

pub fn dream() -> Status {
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

    status_response(Some("Dream recording server"), Some(players))
}
