mod commands;

use serenity::framework::StandardFramework;
use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::*;
use serenity::{async_trait, prelude::*};

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
            let options = &command.data.options;
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(options),
                _ => "Hmm?".to_owned(),
            };
            use serenity::framework::standard::Command;

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

        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| commands::ping::register(command))
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
            .group(&commands::GENERAL_GROUP);

        info!("Kana v{}", build::PKG_VERSION);
        info!("Kana is starting");

        let token = if shadow_rs::is_debug() {
            config.discord_dev_token.clone()
        } else {
            config.discord_prod_token.clone()
        };

        let intents = GatewayIntents::non_privileged()
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let mut client = Client::builder(&token, intents)
            .event_handler(MessageHandler)
            .framework(framework)
            .await
            .expect("Error creating client");

        if let Err(why) = client.start().await {
            error!("An error occured while running the client: {:?}", why);
        }
    }
}
