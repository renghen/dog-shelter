use crate::Settings;
use anyhow::Ok;
use clap::{value_parser, Arg, ArgMatches, Command};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

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

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(sub_matches) = matches.subcommand_matches("serve") {
        let port: u16 = *sub_matches.get_one("port").unwrap_or(&8080);
        start_tokio(port, settings)?
    }

    Ok(())
}

fn start_tokio(port: u16, _setting: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
            let routes = dog_shelter::api::configure();

            let listener = TcpListener::bind(addr).await?;
            axum::serve(listener, routes).await?;

            Ok(())
        })?;
    Ok(())
}
