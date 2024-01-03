mod commands;

use anyhow::Ok;
use clap::{Arg, Command};
use dog_shelter::settings::Settings;
use dotenv::dotenv;

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
    let settings = Settings {};

    let matches = command.get_matches();
    commands::handle(&matches, &settings)?;

    Ok(())
}
