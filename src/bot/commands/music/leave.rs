use crate::bot::commands::KanaCommand;
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    framework::standard::CommandResult,
    model::prelude::{
        interaction::application_command::ApplicationCommandInteraction, GuildId, Message, UserId,
    },
    prelude::*,
};

use super::KanaSongQueue;

pub struct Leave;
impl Leave {
    async fn internal_skip(ctx: &Context, guild_id: Option<GuildId>, user_id: UserId) -> String {
        let guild = match guild_id {
            Some(x) => x.to_guild_cached(&ctx.cache).unwrap(),
            None => return "Something went wrong".to_owned(),
        };

        let channel_id = guild
            .voice_states
            .get(&user_id)
            .unwrap()
            .channel_id
            .unwrap();

        let manager = songbird::get(ctx).await.unwrap().clone();

        let handler = manager.join(guild_id.unwrap(), channel_id).await;
        let mut t = handler.0.lock().await;

        t.leave().await.unwrap();

        let data_read = ctx.data.read().await;
        let mut queues = data_read.get::<KanaSongQueue>().unwrap().lock().await;

        let queue = queues.entry(guild.id).or_default();

        queue.stop();

        "Nara".to_owned()
    }
}

#[async_trait]
impl KanaCommand for Leave {
    async fn run_interaction(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
        Leave::internal_skip(ctx, command.guild_id, command.user.id).await
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("won").description("Nienawidze muzyki")
    }

    async fn run_command(ctx: &Context, msg: &Message) -> CommandResult {
        msg.reply(
            ctx,
            Leave::internal_skip(ctx, msg.guild_id, msg.author.id).await,
        )
        .await
        .unwrap();

        Ok(())
    }
}
