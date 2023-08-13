use valence::{
    network::{
        PlayerSampleEntry,
        ServerListPing::{self, Respond},
    },
    text::IntoText,
};

static MC_VERSION: &str = "1.20.1";

pub fn status_response(s: Option<&str>, players: Option<Vec<PlayerSampleEntry>>) -> ServerListPing {
    let motd = s.unwrap_or("A Minecraft Server");
    let motd = motd.to_string();
    let player_sample = players.unwrap_or(Vec::new());

    Respond {
        online_players: player_sample.len() as i32,
        max_players: 20,
        player_sample,
        description: motd.into_text(),
        favicon_png: &[0; 0],
        version_name: MC_VERSION.to_string(),
        protocol: 763,
    }
}

/// A full response with Valence info or blacklisted info?
pub fn blacklisted<'a>() -> ServerListPing<'a> {
    todo!()
}

pub fn base<'a>() -> ServerListPing<'a> {
    status_response(None, None)
}

pub fn anonymous<'a>(online: i32) -> ServerListPing<'a> {
    let players = (0..=online)
        .map(|_| PlayerSampleEntry {
            name: "Anonymous Player".to_string(),
            id: "00000000-0000-0000-0000-000000000000".parse().unwrap(),
        })
        .collect();

    status_response(None, Some(players))
}

pub fn dream<'a>() -> ServerListPing<'a> {
    let players = vec![
        PlayerSampleEntry {
            name: "Dream".to_string(),
            id: "ec70bcaf-702f-4bb8-b48d-276fa52a780c".parse().unwrap(),
        },
        PlayerSampleEntry {
            name: "GeorgeNotFound".to_string(),
            id: "bd3dd5a4-0438-4699-b2fd-36f518154b41".parse().unwrap(),
        },
        PlayerSampleEntry {
            name: "Sapnap".to_string(),
            id: "c66f7c8a-ed0c-4469-90b0-421d8ff7ca49".parse().unwrap(),
        },
    ];

    status_response(Some("Dream recording server"), Some(players))
}
