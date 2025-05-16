use crate::error::NasError;
use crate::{trace, info, warn, error, fatal, logging, logging::LoggingLevel, logging::logging_function};
use std::sync::{Mutex, MutexGuard};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use std::sync::Arc;
use actix_web::http::StatusCode;
use clap::ArgMatches;
use serde::{Serialize, Deserialize};
use ron;
use actix_web::{get, post, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder};

const DEFAULT_GAME_LIB_PATH: &str = "game_library.ron";
const DEFAULT_SERVER_SETTINGS_PATH: &str = "server_settings.ron";
const DEFAULT_IP_ADDR: &str = "127.0.0.1";
const DEFAULT_IP_PORT: u16 = 55317;
const DEFAULT_GAME_LIBRARY_STR: &str = "[]";


#[derive(Serialize, Deserialize, Debug)]
pub struct ServerSettings {
    #[serde(default)]
    ip: String,
    port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self { Self { ip: DEFAULT_IP_ADDR.to_owned(), port: DEFAULT_IP_PORT} }
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
#[derive(Debug, Deserialize, Serialize)]
pub struct GameLibrary {
    collection: Mutex<Vec<Game>>, // TODO: explore if a hashset is a better choice
}

impl GameLibrary {
    #[allow(dead_code)]
    pub fn new() -> Self { Self { collection: Mutex::new(Vec::new()) } }
}

#[actix_web::main]
pub async fn server(args: &ArgMatches)  -> std::io::Result<()> {
    let path = Path::new(DEFAULT_SERVER_SETTINGS_PATH);
    let server_settings : ServerSettings = get_server_settings(path).unwrap_or_else( |_| {
        info!(&format!("Server settings could not be found at {:?}", path));
        ServerSettings::default()
    });
    if args.get_flag("default") {
        // generate and write the defaults for the server
        let _ = write_server_settings(path, None).unwrap_or_else(|e| {
            error!(&format!("Failed to override settings with {:?}", e)); // TODO: add an early escape if this fails
        });
    }
    if args.get_flag("info") {
        info!(&format!("Server config location at {:?}", path));
        info!(&format!("Server settings are: {:?}", server_settings));
    };
    if args.get_flag("start") {
        info!(&format!("Server started"));
        let game_library_path = PathBuf::from(DEFAULT_GAME_LIB_PATH);
        let game_library_path_str = match game_library_path.to_str() {
            Some(s) => s,
            None => {
                error!(&format!("Failed to load the game library path from {:?}", game_library_path));
                info!(&format!("Falling back to default game library path with {:?}", DEFAULT_GAME_LIB_PATH));
                DEFAULT_GAME_LIB_PATH
            },
        };
        let game_library_file = match fs::read_to_string(game_library_path_str) {
            Ok(s) => s,
            Err(e) => {
                error!(&format!("Failed to read file with: {:?}", e));
                info!(&format!("Falling back to default value with {:?}", DEFAULT_GAME_LIBRARY_STR));
                DEFAULT_GAME_LIBRARY_STR.to_owned()
            },
        };
        let raw_gamelib: Vec<Game> = match ron::from_str::<Vec<Game>>(&game_library_file) {
            Ok(s) => s,
            Err(_) => {
                error!(&format!("Failed to deserialize the game library from: {:?}", game_library_path_str));
                info!(&format!("Falling back to default game library"));
                Vec::new()
            }
        };
        let gamelib = web::Data::new(GameLibrary { collection: Mutex::new(raw_gamelib)}); 
        // let gamelib1 = web::Data::new(GameLibrary::new());ii
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
    info!(&format!("Added {} to in-memory game library", &counter));
    HttpResponse::build(StatusCode::OK).body(format!("{} games have been added", &counter))
}

#[post("/save_library")]
async fn save_library(filelocation: web::Data<PathBuf>, data: web::Data<GameLibrary>) -> impl Responder {
    let lib = match data.collection.lock() {
        Ok(s) => s.clone(),
        Err(_) => return HttpResponse::InternalServerError().body("")
    };
    // the Vec<Game> is used because Mutexes are not serializable
    let game_lib = match ron::to_string::<Vec<Game>>(&lib) {
        Ok(s) => s,
        Err(_) => return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body("Failed to serialize")
    };
    match fs::write(&**filelocation, game_lib) {
        Ok(_) => (),
        Err(_) => return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body("Failed to write to file")
    }
    info!(&format!("Saved in-memory library to {:?}", filelocation.as_path()));
    HttpResponse::build(StatusCode::OK).body("library has been saved")
}
