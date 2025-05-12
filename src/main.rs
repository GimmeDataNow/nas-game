use clap::{Arg, ArgAction, ArgMatches, Command};
use serde::Deserialize;
use toml;
use std::fs;


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
        ).get_matches();
        

    match cmd.subcommand() {
        #[allow(unused_variables)]
        Some(("client", args)) => println!("TODO: show the settings"),
        Some(("server", args)) => {
            if args.get_flag("info") {
              println!("info is set"); 
            };
            if args.get_flag("start") {
              println!("start is set"); 
            };

            server(&args);
        },
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    };
}

#[derive(Deserialize)]
pub struct ServerSettings {
    ip: String,
    port: Option<u16>,
    
}

pub fn server(args: &ArgMatches) {

    // let settings_file = fs::read_to_string("server_config.toml").unwrap_or_else(op); 

    if args.get_flag("info") {
        
    };
    
}
