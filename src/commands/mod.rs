use crate::settings::Settings;
use anyhow::Ok;
use clap::{ArgMatches, Command};

mod createadmin;
mod hello;
mod migrate;
mod serve;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
        .subcommand(createadmin::configure())
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches)?;
    serve::handle(matches, settings)?;
    migrate::handle(matches, settings)?;
    createadmin::handle(matches, settings)?;
    Ok(())
}
