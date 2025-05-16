#![allow(unused_imports)]
mod error;
mod logging;
use error::NasError;
mod server;
use clap::{Arg, ArgAction, ArgMatches, Command};

fn main() {
    let cmd = Command::new("nas-game")
        // .multicall(true) # no need for it yet
        .about("NAS game manager and launcher")
        .version("0.0.1")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .subcommand(Command::new("client").about("starts the client")) // TODO: the client
        .subcommand(
            Command::new("server")
                .about("the server")
                .arg(
                    Arg::new("info")
                        .short('i')
                        .long("info")
                        .action(ArgAction::SetTrue)
                        // .num_args(..1) // should accept info 'level' (if none is provided do default)
                        .help("spits out some info about the server")
                )
                .arg(
                    Arg::new("start")
                        .short('s')
                        .long("start")
                        .action(ArgAction::SetTrue)
                        .help("start the server") // TODO: change it to a sub commmand for additional args
                )
                .arg(
                    Arg::new("default")
                        .long("default")
                        .action(ArgAction::SetTrue)
                        .help("generate default values for the server") // TODO: change it to a sub commmand for additional args
                )
        ).get_matches();
        

    match cmd.subcommand() {
        #[allow(unused_variables)]
        Some(("client", args)) => println!("TODO: show the settings"),
        Some(("server", args)) => {
            let _ = server::server(&args);
        },
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    };
}
