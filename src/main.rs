use clap::{Arg, Command};
use dog_shelter::commands;
use dog_shelter::settings;
use dotenv::dotenv;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let mut command = Command::new("Dog Shelter sample Axum application")
        .version("1.0")
        .author("Renghen renghen@yahoo.com")
        .about("Sample rust app using axum")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("configuration file location")
                .default_value("config.json"),
        );
    command = commands::configure(command);

    let matches = command.get_matches();

    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = settings::Settings::new(config_location, "SHELTER")?;

    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stdout));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // println!(
    //     "db url: {}",
    //     settings
    //         .database
    //         .url
    //         .unwrap_or("missing database url".to_string())
    // );

    // println!(
    //     "log level: {}",
    //     settings.logging.log_level.unwrap_or("info".to_string())
    // );

    commands::handle(&matches, &settings)?;

    Ok(())
}
