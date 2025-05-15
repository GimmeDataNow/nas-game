use crate::error::NasError;
use std::sync::Mutex;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use std::sync::Arc;
use actix_web::http::StatusCode;
use clap::ArgMatches;
use serde::{Serialize, Deserialize};
use ron;
use actix_web::{get, post, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder};

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

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum Launcher {
    Steam,
    Gog,
    EpicGames
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Game {
    launcher: Launcher,
    id: String, 
}

impl Game {
    #[allow(dead_code)]
    pub fn new(launcher: Launcher, id: String) -> Self { Self { launcher, id } }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GameLibrary {
    collection: Mutex<Vec<Game>>, // TODO: explore if a hashset is a better choice
}

impl GameLibrary {
    pub fn new() -> Self { Self { collection: Mutex::new(Vec::new()) } }
}

#[actix_web::main]
pub async fn server(args: &ArgMatches)  -> std::io::Result<()> {
    let path = Path::new("./server_settings.ron");
    let server_settings : ServerSettings = get_server_settings(path).unwrap_or_else( |_| {
        println!("server settings could not be found at {:?}", path);
        ServerSettings::default()
    });

    if args.get_flag("default") {
        // generate and write the defaults for the server
        let _ = write_server_settings(path, None).unwrap_or_else(|e| {
            println!("Failed to print with: {:?}", e); // TODO: add an early escape if this fails
        });
    }
    if args.get_flag("info") {
        // vomit out the info
        println!("The server is starting with the following settings:");
        println!("File location: {:#?}", path);
        println!("{:#?}", server_settings);
    };
    if args.get_flag("start") {
        println!("server started");
        let gamelib = web::Data::new(GameLibrary::new());
        let filelocation = web::Data::new(PathBuf::from("game_library.ron"));
        return HttpServer::new(move || {
            App::new()
                .app_data(gamelib.clone())
                .app_data(filelocation.clone())
                .service(hello)
                .service(echo)
                .service(add_dummy_get)
                .service(add_to_games)
                .service(save_library)
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
#[get("/add_dummy")]
async fn add_dummy_get(data: web::Data<GameLibrary>) -> impl Responder {
    let mut lib = data.collection.lock().unwrap();
    let len = lib.len().to_string().to_owned();
    lib.push(Game::new(Launcher::Steam, len));
    format!("{:?}", lib)
}

#[post("/games")]
async fn add_to_games(data: web::Data<GameLibrary>, games: web::Json<Vec<Game>>) -> impl Responder {
    let mut lib = data.collection.lock().unwrap();
    let mut counter = 0;
    for item in games.into_inner() {
        if !lib.contains(&item) {
            lib.push(item);
            counter += 1;
        }
    }
    println!("{} games have been added", &counter);
    HttpResponse::build(StatusCode::OK).body(format!("{} games have been added", &counter))
}

#[post("/save_library")]
async fn save_library(filelocation: web::Data<PathBuf>, data: web::Data<GameLibrary>) -> impl Responder {
    let lib = data.collection.lock().unwrap();
    let game_lib = match ron::to_string(&*lib) {
        Ok(s) => s,
        Err(_) => return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body("Failed to serialize")
    };
    match fs::write(&**filelocation, game_lib) {
        Ok(_) => (),
        Err(_) => return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body("Failed to write to file")
    }
    println!("saved library");
    HttpResponse::build(StatusCode::OK).body("library has been saved")
}
