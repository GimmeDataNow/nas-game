use crate::error::NasError;
use crate::{trace, info, warn, error, logging::LoggingLevel, logging::logging_function};
use crate::types::{Launcher, Game, GameLibrary};
use clap::ArgMatches;
use std::{fs, env};
use std::sync::Mutex;
use std::path::{Path, PathBuf};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use serde::{Serialize, Deserialize};
use serde_json;
use image::*;
use webp::*;
use steamgriddb_api::{Client, query_parameters::QueryType::Grid};
use reqwest;

const DEFAULT_GAME_LIB_PATH: &str = "game_library.json";
const DEFAULT_SERVER_SETTINGS_PATH: &str = "server_settings.json";
const DEFAULT_IP_ADDR: &str = "127.0.0.1";
const DEFAULT_IP_PORT: u16 = 53317;
const DEFAULT_GAME_LIBRARY_CONTENTS: &str = "[]";

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

pub fn default_cwd() -> PathBuf {
    let path = expand_tilde("~/.local/share/nas-game/server");
    if !path.exists() {
        let _ = std::fs::create_dir_all(&path).map_err(|e| {
            warn!("Failed to create the default directory {:?} with {:?}", std::env::current_dir().unwrap().join(&path),  e);
         });
        info!("Folder {:?} was created since it was missing", path);
    }
    path
}

pub fn prepare_folder<P: AsRef<Path>>(path: P) -> P {
    // just issue a warning but don't error 
    let _ = std::fs::create_dir(&path).map_err(|e| { trace!("Failed to create the directory {:?} with {:?}", std::env::current_dir().unwrap().join(&path),  e); });
    path
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

pub async fn fetch_image(search_for: &str, path_out: &Path) -> Result<(), Box<dyn std::error::Error>> {

    // get api key for steam grid
    let key_env = "STEAM_GRID_API_KEY";
    let key = env::var(&key_env).unwrap_or_else(|_| {
        warn!("STEAM_GRID_API_KEY is not set, using default value of 'key'");
        "key".to_string()
    });
    trace!("STEAM_GRID_API_KEY is {:?}", key);
    
    // stearch for the game
    let client = Client::new(key);
    let games = client.search(search_for).await?;
    let first_game = games.iter().next().ok_or("No games found")?;
    let images = client.get_images_for_id(first_game.id, &Grid(None)).await?;

    // get get the file extensions in a scuffed manner
    let temp = PathBuf::from(&images[0].url);
    let temp_1 = temp.extension().unwrap();
    info!("The file extension is: {:?}", &temp_1);
    let complete_path = path_out.join(search_for.to_owned() + "." + temp_1.to_str().unwrap());
    trace!("The complete path is: {:?}", complete_path);

    // get the image
    let response = reqwest::get(&images[0].url).await?;
    if response.status().is_success() {

        let mut dest = fs::File::create(complete_path)?;
        let bytes = response.bytes().await?;
        let mut content = bytes.as_ref();

        std::io::copy(&mut content, &mut dest)?;
        info!("Image saved as {}.{:?}", search_for, temp_1);
    } else {
        error!("Failed to fetch image: {}", response.status());
    }
    Ok(())
}

/// somehow will not override images for some reason
pub fn optimize_image(path_in: &Path, path_out: &Path, target_dimension: &Option<(u32, u32)>) {

    let file_name = path_in.file_name().unwrap_or(std::ffi::OsStr::new("fail.webp"));
    let img = match image::open(path_in) {
        Ok(i) => i,
        Err(e) => {
            error!("Failed to open image at {:?}: {}", path_in, e);
            return;
        }
    };
    
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
    let out_path = path_out.join(file_name);
    std::fs::write(&out_path, &*webp).unwrap();
}

pub fn optimize_images(path_in: &Path, path_out: &Path) {
    // might error if empty
    let entries = match fs::read_dir(&path_in) {
        Ok(entries) => entries,
        Err(e) => {
            error!("Failed to read input directory {:?} with {:?}", &path_in, e);
            return;
        }
    };
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        match path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("png" | "jpg" | "webp") => {
                info!("File found at: {:?}", &path_in);
                optimize_image(&path, &path_out, &Some((600,900)));
            },
            _ => { warn!("No images were found with the correct file extension"); }
        }
    }
}

#[actix_web::main]
pub async fn server(args: &ArgMatches)  -> std::io::Result<()> {
    // set and get the default cwd
    let cwd = default_cwd();
    std::env::set_current_dir(&cwd).unwrap();
    info!("CWD is: {:?}", std::env::current_dir().unwrap());

    // get server settings     
    let server_settings_path = std::env::current_dir().unwrap().join(DEFAULT_SERVER_SETTINGS_PATH);
    info!("Server settings path is {:?}", server_settings_path);
    let server_settings : ServerSettings = get_server_settings(&server_settings_path).unwrap_or_else( |_| {
        warn!("Server settings could not be found at {:?}", server_settings_path);
        ServerSettings::default()
    });

    // gen default server settings
    if args.get_flag("default") {
        let _ = write_server_settings(&server_settings_path, None).unwrap_or_else(|e| {
            error!("Failed to override settings with {:?}", e); // TODO: add an early escape if this fails
        });
    }
    // vomit out data about the server
    if args.get_flag("info") {
        info!("Server config location at {:?}", std::env::current_dir().unwrap().join(DEFAULT_SERVER_SETTINGS_PATH));
        info!("Server settings are: {:?}", server_settings);
    };

    // optimize images
    // does this by taking in a input directory and an output directory
    if args.get_flag("optimize-images") {

        // create folders
        prepare_folder("images");
        std::env::set_current_dir("images").unwrap();
        info!("CWD is: {:?}", std::env::current_dir().unwrap());
        prepare_folder("non-optimized");
        prepare_folder("optimized");

        let (path_in, path_out) = (std::env::current_dir().unwrap().join("non-optimized"), std::env::current_dir().unwrap().join("optimized"));
        info!("Optimizing images at: {:?} -> {:?}", &path_in, &path_out);
        
        optimize_images(&path_in, &path_out);
        std::env::set_current_dir(&cwd).unwrap();
        info!("CWD is now {:?}", std::env::current_dir().unwrap());
    };

    // try to download images from steamgrid
    if args.get_flag("download-images") {
        info!("Trying to download images");
        // this will ERROR if images/non-optimized doesn't exist
        std::env::set_current_dir(&cwd.join("images/non-optimized")).unwrap();

        // save to the current directory
        let _ = fetch_image("celeste", Path::new("")).await.map_err(|e| { error!("Failed to fetch image with: {:?}", e); });
    };
    
    if args.get_flag("start") {
        info!("Server started");
        let game_library_path = PathBuf::from(DEFAULT_GAME_LIB_PATH);
        let game_library_file = match fs::read_to_string(&game_library_path) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to read game library file with: {:?}", e);
                info!("Falling back to default value with {:?}", DEFAULT_GAME_LIBRARY_CONTENTS);
                DEFAULT_GAME_LIBRARY_CONTENTS.to_owned()
            },
        };
        let raw_gamelib: Vec<Game> = match serde_json::from_str::<Vec<Game>>(&game_library_file) {
            Ok(s) => s,
            Err(_) => {
                error!("Failed to deserialize the game library from: {:?}", game_library_path);
                info!("Falling back to default game library");
                Vec::new()
            }
        };
        let gamelib = web::Data::new(GameLibrary { collection: Mutex::new(raw_gamelib)}); 
        let filelocation = web::Data::new(PathBuf::from(DEFAULT_GAME_LIB_PATH));
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
    info!("Added {} to in-memory game library", &counter);
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
    info!("Saved in-memory library to {:?}", filelocation.as_path());
    HttpResponse::build(StatusCode::OK).body("library has been saved")
}
