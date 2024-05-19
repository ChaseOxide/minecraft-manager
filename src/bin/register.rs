use discord_client::{
    DiscordClient, DiscordCommand, DiscordCommandOption, DiscordCommandOptionChoice,
    DiscordCommandOptionType,
};
use dotenv_codegen::dotenv;

#[path = "../discord_client.rs"]
mod discord_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DiscordClient {
        app_id: String::from(dotenv!("APP_ID")),
        discord_token: String::from(dotenv!("DISCORD_TOKEN")),
        guild_id: String::from(dotenv!("GUILD_ID")),
    };

    install_command(
        &client,
        &DiscordCommand {
            name: String::from("whitelist"),
            description: String::from("プレイヤーをホワイトリストに追加する"),
            options: vec![DiscordCommandOption {
                typed: DiscordCommandOptionType::String,
                name: String::from("プレイヤー名"),
                description: String::from("対象プレイヤー"),
                required: true,
                choices: None,
            }],
        },
    )
    .await?;

    install_command(
        &client,
        &DiscordCommand {
            name: String::from("gamemode"),
            description: String::from("ゲームモード"),
            options: vec![
                DiscordCommandOption {
                    typed: DiscordCommandOptionType::String,
                    name: String::from("ゲームモード"),
                    description: String::from("ゲームモード設定"),
                    required: true,
                    choices: Some(vec![
                        DiscordCommandOptionChoice {
                            name: String::from("サバイバルモード"),
                            value: String::from("survival"),
                        },
                        DiscordCommandOptionChoice {
                            name: String::from("クリエイティブモード"),
                            value: String::from("creative"),
                        },
                    ]),
                },
                DiscordCommandOption {
                    typed: DiscordCommandOptionType::String,
                    name: String::from("プレイヤー名"),
                    description: String::from("対象プレイヤー"),
                    required: true,
                    choices: None,
                },
            ],
        },
    )
    .await?;

    Ok(())
}

async fn install_command(
    client: &DiscordClient,
    cmd: &DiscordCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    let res = client.install_command(cmd).await?;

    let status = res.status().as_u16();
    let data = res.json::<serde_json::Value>().await?;

    if status >= 400 {
        return Err(data.to_string())?;
    }

    println!("{}", data.to_string());
    Ok(())
}
