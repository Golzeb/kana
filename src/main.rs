mod bot;
use bot::Bot;
use fern::colors::{Color, ColoredLevelConfig};

fn init_logger() {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
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
