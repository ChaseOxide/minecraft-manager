use axum::http::HeaderMap;
use ed25519_dalek::{Signature, VerifyingKey};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum DiscordInteractionResponseType {
    Pong = 1,
    ChannelMessageWithSource = 4,
}

#[derive(Serialize)]
pub struct DiscordInteractionResponseData {
    pub content: String,
}

#[derive(Serialize)]
pub struct DiscordInteractionResponse {
    #[serde(rename(serialize = "type"))]
    pub typed: DiscordInteractionResponseType,
    pub data: Option<DiscordInteractionResponseData>,
}

#[derive(serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum DiscordInteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

#[derive(Deserialize)]
pub struct DiscordInteractionDataOption {
    pub name: String,

    pub value: String,
}

#[derive(Deserialize)]
pub struct DiscordInteractionData {
    pub name: String,

    pub options: Vec<DiscordInteractionDataOption>,
}

#[derive(Deserialize)]
pub struct DiscordInteraction {
    #[serde(rename(deserialize = "type"))]
    pub typed: DiscordInteractionType,

    pub data: DiscordInteractionData,
}

impl DiscordInteraction {
    pub fn parse<S: AsRef<str> + Display>(
        headers: HeaderMap,
        body: S,
        public_key: VerifyingKey,
    ) -> Result<DiscordInteraction, StatusCode> {
        let signature = headers.get("X-Signature-Ed25519");
        let timestamp = headers.get("X-Signature-Timestamp");

        let signature = match signature {
            Some(x) => {
                let x = hex::decode(x).or(Err(StatusCode::UNAUTHORIZED))?;
                let x = Signature::from_slice(&x);
                x.or(Err(StatusCode::UNAUTHORIZED))
            }
            None => Err(StatusCode::UNAUTHORIZED),
        }?;
        let timestamp = match timestamp {
            Some(x) => {
                let x = x.to_str();
                x.or(Err(StatusCode::UNAUTHORIZED))
            }
            None => Err(StatusCode::UNAUTHORIZED),
        }?;

        public_key
            .verify_strict(format!("{}{}", timestamp, body).as_bytes(), &signature)
            .or(Err(StatusCode::UNAUTHORIZED))?;

        match serde_json::from_str(body.as_ref()) {
            Ok(x) => Ok(x),
            Err(err) => {
                println!("{}\n{}", err, body);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}
