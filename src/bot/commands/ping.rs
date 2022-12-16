use super::KanaCommand;
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    framework::standard::CommandResult,
    model::prelude::{interaction::application_command::ApplicationCommandInteraction, Message},
    prelude::*,
};

pub struct Ping;

impl Ping {
    fn internal_ping() -> String {
        "Pong".to_owned()
    }
}

#[async_trait]
impl KanaCommand for Ping {
    async fn run_interaction(_ctx: &Context, _command: &ApplicationCommandInteraction) -> String {
        Ping::internal_ping()
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("ping").description("A ping command")
    }

    async fn run_command(ctx: &Context, msg: &Message) -> CommandResult {
        msg.reply(ctx, Ping::internal_ping()).await.unwrap();

        Ok(())
    }
}
