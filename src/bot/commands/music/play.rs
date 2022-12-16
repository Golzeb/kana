use crate::bot::commands::{get_string_fom_option, KanaCommand};
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    framework::standard::{Args, CommandResult, Delimiter},
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::ApplicationCommandInteraction, GuildId, Message, UserId,
    },
    prelude::*,
};

use super::KanaSongQueue;

pub struct Play;
impl Play {
    async fn internal_play(
        ctx: &Context,
        guild_id: Option<GuildId>,
        user_id: UserId,
        url: String,
    ) -> String {
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

        let source = match songbird::ytdl(url.clone()).await {
            Ok(source) => source,
            Err(_) => return "Nie ma takiej muzy".to_owned(),
        };

        let data_read = ctx.data.read().await;
        let mut queues = data_read.get::<KanaSongQueue>().unwrap().lock().await;

        let queue = queues.entry(guild.id).or_default();

        queue.add_source(source, &mut t);

        format!("Daje {}", url)
    }
}

#[async_trait]
impl KanaCommand for Play {
    async fn run_interaction(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
        let url = match get_string_fom_option(&command.data.options, 0) {
            Some(x) => x.to_owned(),
            None => return "?".to_owned(),
        };

        Play::internal_play(ctx, command.guild_id, command.user.id, url).await
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("dawaj")
            .description("Daje muzyke d - _ - b")
            .create_option(|option| {
                option
                    .name("url")
                    .description("URL MUZY YOUTUBE")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    }

    async fn run_command(ctx: &Context, msg: &Message) -> CommandResult {
        let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);

        let args_vec = args
            .iter::<String>()
            .map(|e| e.unwrap_or("".to_owned()))
            .collect::<Vec<String>>();

        msg.reply(
            ctx,
            Play::internal_play(ctx, msg.guild_id, msg.author.id, args_vec[1].clone()).await,
        )
        .await
        .unwrap();

        Ok(())
    }
}
