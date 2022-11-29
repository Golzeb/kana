mod commands;

use commands::KanaCommand;
use serenity::framework::StandardFramework;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::*;
use serenity::{async_trait, prelude::*};
use songbird::SerenityInit;

use serde::Deserialize;

use log::{debug, error, info, warn};

use shadow_rs::shadow;
shadow!(build);

const PREFIX: &'static str = "!";

#[derive(Deserialize, Debug)]
pub struct Config {
    discord_dev_token: String,
    discord_prod_token: String,
}

struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::Ping::run_interaction(&command),
                _ => "Hmm?".to_owned(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                error!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        info!("Kana is running");

        let _commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| commands::ping::Ping::register(command))
        })
        .await;
    }
}

pub struct Bot {}

impl Bot {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(&self, config: Config) {
        let framework = StandardFramework::new()
            .configure(|c| c.prefix(PREFIX))
            .group(&commands::GENERAL_GROUP)
            .group(&commands::MUSIC_GROUP);

        if build::BRANCH == "devel" {
            warn!("RUNNING DEVELOPER VERSION\nIT SHOULDN'T BE USED IN PRODUCTION");
            info!("branch {} commit {}", build::BRANCH, build::SHORT_COMMIT);
        }
        info!("Kana version {}", build::PKG_VERSION);
        info!("Kana is starting");

        let token = if shadow_rs::is_debug() {
            config.discord_dev_token.clone()
        } else {
            config.discord_prod_token.clone()
        };

        let intents = GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_VOICE_STATES;

        let mut client = Client::builder(&token, intents)
            .event_handler(MessageHandler)
            .framework(framework)
            .register_songbird()
            .await
            .expect("Error creating client");

        if let Err(why) = client.start().await {
            error!("An error occured while running the client: {:?}", why);
        }
    }
}
