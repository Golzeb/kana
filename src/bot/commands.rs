use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::{interaction::application_command::ApplicationCommandInteraction, Message},
    prelude::Context,
};

#[async_trait]
pub trait KanaCommand {
    fn run_interaction(command: &ApplicationCommandInteraction) -> String;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    async fn run_command(ctx: &Context, msg: &Message) -> CommandResult;
}

pub mod ping;

#[group]
#[commands(ping)]
pub struct General;

#[command("ping")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    ping::Ping::run_command(ctx, msg).await.unwrap();

    Ok(())
}

pub mod music;

#[group]
#[commands(join)]
pub struct Music;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}
