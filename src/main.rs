mod error;
use error::NasError;
use clap::{Arg, ArgAction, ArgMatches, Command};
use serde::{Deserialize, Serialize};
use ron;
use std::{fs, path::Path};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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
            let _ = server(&args);
        },
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerSettings {
    #[serde(default)]
    ip: String,
    port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self { Self { ip: "127.0.0.1".to_owned(), port: 53317} }
}

pub fn get_server_settings(path: &Path) -> Result<ServerSettings, NasError> {
    let file = fs::read_to_string(path)?;
    ron::from_str::<ServerSettings>(&file).map_err(|_| NasError::FailedToParse)
}

pub fn write_server_settings(path: &Path, settings: Option<ServerSettings>) -> Result<(), NasError> {
    let settings = settings.unwrap_or_default();
    let settings_serialized = ron::to_string(&settings).map_err(|_| NasError::FailedToSerialize)?;
    fs::write(path, settings_serialized).map_err(|_| NasError::FailedToWrite)?;
    Ok(())
}

#[actix_web::main]
pub async fn server(args: &ArgMatches)  -> std::io::Result<()> {
    let path = Path::new("server_settings.ron");
    let server_settings : ServerSettings = get_server_settings(path).unwrap_or_else( |_| {
        println!("server settings could not be found at {:?}", path);
        ServerSettings::default()
    });

    if args.get_flag("default") {
        // generate and write the defaults for the server
        let _ = write_server_settings(Path::new("./server_settings.ron"), None);
    }
    if args.get_flag("info") {
        // vomit out the info
        println!("The server is starting with the following settings:");
        println!("File location: {:#?}", path);
        println!("{:#?}", server_settings);
    };
    if args.get_flag("start") {
        println!("server started");
        return HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
        })
        .bind((server_settings.ip, server_settings.port))?
        .run()
        .await;              
    };
    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
