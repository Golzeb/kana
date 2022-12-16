use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::{
        interaction::application_command::{
            ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
        },
        Message,
    },
    prelude::Context,
};

pub fn get_string_fom_option(options: &[CommandDataOption], index: usize) -> Option<&str> {
    let option = options.get(index).unwrap().resolved.as_ref().unwrap();

    if let CommandDataOptionValue::String(string) = option {
        Some(string)
    } else {
        None
    }
}

#[async_trait]
pub trait KanaCommand {
    async fn run_interaction(ctx: &Context, command: &ApplicationCommandInteraction) -> String;
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
#[commands(music_join, music_skip, music_leave)]
pub struct Music;

#[command("dawaj")]
#[only_in(guilds)]
async fn music_join(ctx: &Context, msg: &Message) -> CommandResult {
    music::play::Play::run_command(ctx, msg).await.unwrap();

    Ok(())
}

#[command("jazda")]
#[only_in(guilds)]
async fn music_skip(ctx: &Context, msg: &Message) -> CommandResult {
    music::skip::Skip::run_command(ctx, msg).await.unwrap();

    Ok(())
}

#[command("won")]
#[only_in(guilds)]
async fn music_leave(ctx: &Context, msg: &Message) -> CommandResult {
    music::leave::Leave::run_command(ctx, msg).await.unwrap();

    Ok(())
}
