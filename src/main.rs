mod bot;
use bot::Bot;
use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info, trace, warn};
use shadow_rs::shadow;

shadow!(build);

fn init_logger() {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            let message_string = message.to_string();
            let message_split: Vec<&str> = message_string.split('\n').collect();

            let head = message_split[0];
            let tail = message_split[1..].join("\n");

            out.finish(format_args!(
                "{} | {:10} | {:5} | {}",
                chrono::Local::now().format("%Y-%m-%d | %H:%M:%S"),
                record.target().to_string().split("::").last().unwrap(),
                colors.color(record.level()),
                head
            ));

            if tail.len() > 0 {
                match record.level() {
                    log::Level::Error => error!("{}", tail),
                    log::Level::Warn => warn!("{}", tail),
                    log::Level::Info => info!("{}", tail),
                    log::Level::Debug => debug!("{}", tail),
                    log::Level::Trace => trace!("{}", tail),
                }
            }
        })
        .level(if shadow_rs::is_debug() {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Debug
        })
        .level_for("rustls", log::LevelFilter::Off)
        .level_for("serenity", log::LevelFilter::Off)
        .level_for("tracing", log::LevelFilter::Off)
        .level_for("h2", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

#[tokio::main]
async fn main() {
    init_logger();

    let config: bot::Config = {
        let config_string =
            std::fs::read_to_string("config.json").expect("Couldn't load config file");
        serde_json::from_str(&config_string).expect("Wrong file structure")
    };

    let bot = Bot::new();
    bot.start(config).await;
}
