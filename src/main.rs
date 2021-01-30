use std::env;
use std::process;

mod arguments;
mod display;
mod subcommands;

use cobweb_core::*;

use clap::{App, AppSettings, crate_version, crate_authors};

fn main() {
    let args = App::new("cobweb")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequired)
        .subcommand(arguments::init().display_order(0))
        .subcommand(arguments::list().display_order(1))
        .subcommand(arguments::open().display_order(2))
        .subcommand(arguments::edit().display_order(3))
        .subcommand(arguments::close().display_order(4))
        .subcommand(arguments::remove().display_order(5))
        .subcommand(arguments::config().display_order(6))
        .get_matches();

    let working_dir = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Can't retrieve working directory: {}", e);
            process::exit(-1);
        }
    };

    if let Some(args) = args.subcommand_matches("init") {
        subcommands::init(args, &working_dir);
    }

    if let Some(args) = args.subcommand_matches("list") {
        subcommands::list(args, &working_dir);
    }

    if let Some(args) = args.subcommand_matches("open") {
        subcommands::open(args, &working_dir);
    }

    if let Some(args) = args.subcommand_matches("edit") {
        subcommands::edit(args, &working_dir);
    }

    if let Some(args) = args.subcommand_matches("close") {
        subcommands::close(args, &working_dir);
    }

    if let Some(args) = args.subcommand_matches("remove") {
        subcommands::remove(args, &working_dir);
    }

    if let Some(args) = args.subcommand_matches("config") {
        subcommands::config(args, &working_dir);
    }
}
// TODO: remove usage of ticket use issue instead
// TODO: implement exit code macro
