use clap::{value_parser, Arg, ArgMatches, Command};

pub fn configure() -> Command {
    Command::new("serve")
        .about("Starting Axum Http Server")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("TCP Port to listen on")
                .default_value("8080")
                .value_parser(value_parser!(u16)),
        )
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(sub_matches) = matches.subcommand_matches("serve") {
        let port: u16 = *sub_matches.get_one("port").unwrap_or(&8080);
        println!("TBD: Start the webserver on port {}", port)
    }

    Ok(())
}
