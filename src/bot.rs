use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::framework::StandardFramework;
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
    async fn ready(&self, _: Context, _ready: Ready) {
        info!("Kana is running");
    }
}

#[group]
#[commands(ping)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await.unwrap();

    Ok(())
}

pub struct Bot {}

impl Bot {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(&self, config: Config) {
        let framework = StandardFramework::new()
            .configure(|c| c.prefix(PREFIX))
            .group(&GENERAL_GROUP);

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
