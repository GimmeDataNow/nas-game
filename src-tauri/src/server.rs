use crate::error::NasError;
use crate::{trace, info, warn, error, fatal, logging, logging::LoggingLevel, logging::logging_function};
use crate::types::{Launcher, Game, GameLibrary};
use std::sync::{Mutex, MutexGuard};
use std::path::{Path, PathBuf};
use std::{ffi, fs};
use std::collections::HashMap;
use std::sync::Arc;
use std::env;
use std::ffi::OsStr;
use actix_web::http::StatusCode;
use clap::ArgMatches;
use serde::{Serialize, Deserialize};
// use ron;
use serde_json;
use actix_web::{get, post, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use image::*;
use image::open;
use webp::*;

const DEFAULT_GAME_LIB_PATH: &str = "game_library.json";
const DEFAULT_SERVER_SETTINGS_PATH: &str = "server_settings.json";
const DEFAULT_IP_ADDR: &str = "127.0.0.1";
const DEFAULT_IP_PORT: u16 = 53317;
const DEFAULT_GAME_LIBRARY_STR: &str = "[]";
static DEFAULT_SERVER_PATH: &str = "~/.local/share/nas-game/server/";


#[derive(Serialize, Deserialize, Debug)]
pub struct ServerSettings {
    #[serde(default)]
    ip: String,
    port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self { Self { ip: DEFAULT_IP_ADDR.to_owned(), port: DEFAULT_IP_PORT} }
}


fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

pub fn prepare_folder(path: &Path) -> Result<PathBuf, NasError>{
    let mut new_dir = expand_tilde("~/.local/share/nas-game");
    // let mut new_dir = PathBuf::from("~/.local/share/nas-game");
    // if !new_dir.exists() { std::fs::create_dir_all(&new_dir).map_err(|_| NasError::InvalidPath)?; }
    new_dir.push(path.to_str().unwrap());
    println!("prep {:?}", new_dir);
    if !new_dir.exists() { std::fs::create_dir_all(&new_dir).map_err(|_| NasError::InvalidPath)?; }
    return Ok(new_dir)
}

pub fn get_server_settings(path: &Path) -> Result<ServerSettings, NasError> {
    let file = fs::read_to_string(path)?;
    serde_json::from_str::<ServerSettings>(&file).map_err(|_| NasError::FailedToParse)
}

pub fn write_server_settings(path: &Path, settings: Option<ServerSettings>) -> Result<(), NasError> {
    let settings = settings.unwrap_or_default();
    let settings_serialized = serde_json::to_string(&settings).map_err(|_| NasError::FailedToSerialize)?;
    fs::write(path, settings_serialized).map_err(|_| NasError::FailedToWrite)?;
    Ok(())
}

/// somehow will not override images for some reason
pub fn optimize_image(path_in: &Path, path_out: &Path, target_dimension: &Option<(u32, u32)>) {
    // let old_file_name = PathBuf::new().push(path_in).;
    let file_name = path_in.file_name().unwrap_or(std::ffi::OsStr::new("fail.webp"));
    println!("path_in_op: {:?}", path_in);
    let img = match image::open(path_in) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Failed to open image {:?}: {}", path_in, e);
            return;
        }
    };
    // let (w, h) = img.dimensions();
    let (w, h) = target_dimension.unwrap_or_else(|| img.dimensions());

    let size_factor = 1.0;
    let img: DynamicImage = image::DynamicImage::ImageRgba8(imageops::resize(
        &img,
        (w as f64 * size_factor) as u32,
        (h as f64 * size_factor) as u32,
        imageops::FilterType::Triangle,
    ));

    // Create the WebP encoder for the above image
    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    // Encode the image at a specified quality 0-100
    let webp: WebPMemory = encoder.encode(90f32);
    // Define and write the WebP-encoded file to a given path
    let out_path = path_out.to_str().unwrap().to_owned() + file_name.to_str().unwrap();
    std::fs::write(&out_path, &*webp).unwrap();
}

pub fn optimize_images(path_in: &Path, path_out: &Path) {
    let path_in_p = prepare_folder(path_in).unwrap();
    let path_out_p = prepare_folder(path_out).unwrap();
    // might error if empty
    //
    let entries = match fs::read_dir(&path_in_p) {
        Ok(entries) => entries,
        Err(e) => {
            println!("path_in: {:?}", &path_in_p);
            eprintln!("Failed to read input directory: {}", e);
            return;
        }
    };
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("wowie {:?}", path);
        match path.extension().and_then(OsStr::to_str) {
            Some("png" | "jpg" | "webp") => { println!("many: {:?}", &path_in);optimize_image(&path, &path_out_p, &Some((600,900))); },
            _ => { println!("nopee"); }
        }
    }
}

#[actix_web::main]
pub async fn server(args: &ArgMatches)  -> std::io::Result<()> {
    let cwd = prepare_folder(Path::new("server/")).unwrap();
    std::env::set_current_dir(cwd).unwrap();
    info!(&format!("CWD is: {:?}", std::env::current_dir()));
     
    let path = Path::new(DEFAULT_SERVER_SETTINGS_PATH);
    let server_settings : ServerSettings = get_server_settings(path).unwrap_or_else( |_| {
        warn!(&format!("Server settings could not be found at {:?}", path));
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
    if args.get_flag("optimize-images") {
        info!(&format!("Optimizing images at: {:?}", server_settings));
        optimize_images(Path::new("images/non-optimized/"), Path::new("images/optimized/"));
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
                error!(&format!("Failed to read game library file with: {:?}", e));
                info!(&format!("Falling back to default value with {:?}", DEFAULT_GAME_LIBRARY_STR));
                DEFAULT_GAME_LIBRARY_STR.to_owned()
            },
        };
        let raw_gamelib: Vec<Game> = match serde_json::from_str::<Vec<Game>>(&game_library_file) {
            Ok(s) => s,
            Err(_) => {
                error!(&format!("Failed to deserialize the game library from: {:?}", game_library_path_str));
                info!(&format!("Falling back to default game library"));
                Vec::new()
            }
        };
        let gamelib = web::Data::new(GameLibrary { collection: Mutex::new(raw_gamelib)}); 
        let filelocation = web::Data::new(PathBuf::from("game_library.json"));
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
    let game_lib = match serde_json::to_string::<Vec<Game>>(&lib) {
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
