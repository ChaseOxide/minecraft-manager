use std::net::SocketAddr;

use axum::{
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use discord_interaction::{
    DiscordInteractionResponse, DiscordInteractionResponseData, DiscordInteractionResponseType,
    DiscordInteractionType,
};
use dotenv_codegen::dotenv;
use ed25519_dalek::VerifyingKey;
use reqwest::StatusCode;

use crate::minecraft::minecraft_cmd;

mod discord_interaction;
mod minecraft;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { StatusCode::NO_CONTENT }))
        .route(
            "/interactions",
            post({
                let public_key = dotenv!("PUBLIC_KEY");
                let public_key = hex::decode(public_key).expect("PUBLIC_KEY malformatted");
                let public_key = VerifyingKey::from_bytes(&public_key[..].try_into().unwrap())
                    .expect("PUBLIC_KEY malformatted");
                move |h, b| discord_interactions(h, b, public_key)
            }),
        );

    let config = axum_server::tls_rustls::RustlsConfig::from_pem_file(
        dotenv!("CERT_PATH"),
        dotenv!("KEY_PATH"),
    )
    .await
    .expect("TLS config failed");

    println!("Server running on http://localhost:3000");
    axum_server::bind_rustls(SocketAddr::from(([0, 0, 0, 0], 3000)), config)
        // axum_server::bind(SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(app.into_make_service())
        .await
        .expect("serve port 3000 failed");
}

async fn discord_interactions(
    headers: HeaderMap,
    body: String,
    public_key: VerifyingKey,
) -> Result<impl IntoResponse, StatusCode> {
    let body = discord_interaction::DiscordInteraction::parse(headers, body, public_key)?;

    match body.typed {
        DiscordInteractionType::Ping => Ok(Json(DiscordInteractionResponse {
            typed: DiscordInteractionResponseType::Pong,
            data: None,
        })),
        DiscordInteractionType::ApplicationCommand => match body.data.name.as_str() {
            "whitelist" => {
                let player_name = &body.data.options[0].value;
                minecraft_cmd(format!("whitelist add {}", player_name))
                    .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;
                Ok(Json(DiscordInteractionResponse {
                    typed: DiscordInteractionResponseType::ChannelMessageWithSource,
                    data: Some(DiscordInteractionResponseData {
                        content: format!("{}をホワイトリストに入りました", player_name),
                    }),
                }))
            }
            "gamemode" => match body.data.options[0].value.as_str() {
                mode @ ("survival" | "creative") => {
                    let player_name = &body.data.options[1].value;
                    minecraft_cmd(format!("gamemode {} {}", mode, player_name))
                        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;
                    let mode_label = match mode {
                        "survival" => "サバイバルモード",
                        "creative" => "クリエイティブモード",
                        _ => mode,
                    };
                    Ok(Json(DiscordInteractionResponse {
                        typed: DiscordInteractionResponseType::ChannelMessageWithSource,
                        data: Some(DiscordInteractionResponseData {
                            content: format!(
                                "{}のゲームモードを{}にしました",
                                player_name, mode_label
                            ),
                        }),
                    }))
                }
                _ => Err(StatusCode::UNPROCESSABLE_ENTITY),
            },
            _ => Err(StatusCode::UNPROCESSABLE_ENTITY),
        },
    }
}
