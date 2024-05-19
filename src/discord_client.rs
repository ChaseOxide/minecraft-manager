use serde::Serialize;
use std::future::Future;

#[derive(serde_repr::Serialize_repr)]
#[repr(u8)]
pub enum DiscordCommandOptionType {
    String = 3,
}

#[derive(Serialize)]
pub struct DiscordCommandOptionChoice {
    pub name: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct DiscordCommandOption {
    #[serde(rename(serialize = "type"))]
    pub typed: DiscordCommandOptionType,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub choices: Option<Vec<DiscordCommandOptionChoice>>,
}

#[derive(Serialize)]
pub struct DiscordCommand {
    pub name: String,
    pub description: String,
    pub options: Vec<DiscordCommandOption>,
}

pub struct DiscordClient {
    pub app_id: String,
    pub discord_token: String,
    pub guild_id: String,
}

impl DiscordClient {
    pub fn install_command(
        &self,
        command: &DiscordCommand,
    ) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
        reqwest::Client::new()
            .post(format!(
                "https://discord.com/api/v10/applications/{}/guilds/{}/commands",
                self.app_id, self.guild_id,
            ))
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bot {}", self.discord_token),
            )
            .json(&command)
            .send()
    }
}
