use clap::{Arg, Command};

pub fn main() {
    let command = Command::new("Dog Shelter sample Axum application")
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

    let _command_matches = command.get_matches();

    println!("Hello, world!");
}
