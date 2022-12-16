use crate::bot::commands::KanaCommand;
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    framework::standard::CommandResult,
    model::prelude::{
        interaction::application_command::ApplicationCommandInteraction, GuildId, Message,
    },
    prelude::*,
};

use super::KanaSongQueue;

pub struct Skip;
impl Skip {
    async fn internal_skip(ctx: &Context, guild_id: Option<GuildId>) -> String {
        let guild = match guild_id {
            Some(x) => x.to_guild_cached(&ctx.cache).unwrap(),
            None => return "Something went wrong".to_owned(),
        };

        let data_read = ctx.data.read().await;
        let mut queues = data_read.get::<KanaSongQueue>().unwrap().lock().await;

        let queue = queues.entry(guild.id).or_default();

        queue.skip().unwrap();

        "Jedziemy panowie".to_owned()
    }
}

#[async_trait]
impl KanaCommand for Skip {
    async fn run_interaction(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
        Skip::internal_skip(ctx, command.guild_id).await
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("jazda").description("Nienawidze muzyki")
    }

    async fn run_command(ctx: &Context, msg: &Message) -> CommandResult {
        msg.reply(ctx, Skip::internal_skip(ctx, msg.guild_id).await)
            .await
            .unwrap();

        Ok(())
    }
}
