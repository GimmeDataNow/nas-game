//! This crate is for handling the server side code of the application.
//! This means that this crate orchestrates which functions should be called,
//! it also defines the API. 
//!
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


/// All of the relevant server settings as a struct
///
/// `ServerSettings` contains all of the relevant data for the server.
/// This includes IP, IP PORT, and maybe more.
/// # Arguments
/// `ìp` - This is the IP the server will be listening on
/// `port` - This is the PORT the server will be listening on
/// # IPv4 vs IPv6
/// The server doesn't handle IPv6 just yet as such the struct does
/// not support it
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerSettings {
    #[serde(default)]
    ip: String,
    port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self { Self { ip: DEFAULT_IP_ADDR.to_owned(), port: DEFAULT_IP_PORT} }
}

/// Expands the `~/` expression for relative paths on linux-like systems
///
/// # Return
/// It returns a `PathBuf` which is either the absolute path or it
/// simply returns the given path without modification if there is
/// no `~/` present at the start of the the input path.
///
/// # Examples
/// ```
/// "~/.local" -> "/home/$USER/.local"
/// "example/" -> "example"
/// "example" -> "example"
/// ```
fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

/// Sets up the default working directory for the server.
///
/// It first generates where the working directory should be.
/// Then it checks if the path exists and if it doesn't it
/// creates all the paths recursively to avoid any errors.
/// It will print an error if the path creating fails. This
/// is because the `create_dir_all()` function will error if
/// the path already exists. This of course shouldn't happen
/// since it checks for the path prior thus the error will
/// likely be of a different kind.
pub fn default_cwd() -> PathBuf {
    let path = expand_tilde("~/.local/share/nas-game/server");
    if !path.exists() {
        if let Err(e) = std::fs::create_dir_all(&path) {
            error!("Failed to create the default directory {:?} with {:?}", std::env::current_dir().unwrap().join(&path),  e);
            return path;
        };
        info!("Folder {:?} was created since it was missing", &path);
    }
    path
}

/// Creates a dir in the working directory 
///
/// Creates a dir in the current working directory.
/// Should `std::fs::create_dir()` fail then the error will
/// be logged using trace. It is not uncommon for the function
/// to error since it will often be called in duplicate
/// contexts.
///
/// # Errors
/// This will only log the error and not return the error itself. 
///
/// # TODO
/// Improve the logging corresponding to the error type that
/// `std::fs::create_dif()` returns.
pub fn prepare_folder<P: AsRef<Path>>(path: P) -> P {
    // just issue a warning but don't error 
    let _ = std::fs::create_dir(&path).map_err(|e| { trace!("Failed to create the directory {:?} with {:?}", std::env::current_dir().unwrap().join(&path),  e); });
    path
} 

/// Read the server settings from the specified file
///
/// This reads the file contents and then tries to parse the
/// results into a `ServerSettings` struct.
///
/// # Errors
/// This fuction can error both at the file read and the parsing
/// part. Should the `fs::read_to_sting()` fail then this function
/// will map the error to be `NasError::Ignore`. The parsing error
/// will be mapped to `NasError::FailedToReadFile`.
pub fn get_server_settings(path: &Path) -> Result<ServerSettings, NasError> {
    let file = fs::read_to_string(path).map_err(|_| NasError::FailedToReadFile)?;
    serde_json::from_str::<ServerSettings>(&file).map_err(|_| NasError::FailedToParse)
}

/// Write server settings to a file
///
/// This will write the server settings if provided to the
/// specified path. Should the server settings not be providied
/// then it will fall back to writing the default server settings
///
/// # Errors
/// It can only error at the serialization or at the file writing.
/// As such the errors can only be `NasError::FailedToSerialize` or
/// `NasError::FailedToWrite`
pub fn write_server_settings(path: &Path, settings: Option<ServerSettings>) -> Result<(), NasError> {
    let settings = settings.unwrap_or_default();
    let settings_serialized = serde_json::to_string(&settings).map_err(|_| NasError::FailedToSerialize)?;
    fs::write(path, settings_serialized).map_err(|_| NasError::FailedToWrite)?;
    Ok(())
}

/// Fetch the image from the steam grid api based on the input
///
/// This function will attempt to fetch and save an image from
/// the steam grid api. It uses the `search_for` parameter to
/// filter the results. The first result from the results vec
/// is used to then fetch the image and save it to the `path_out`
/// directory
///
/// # Errors
/// This fucition has many points of failure. So many that it
/// might be best to split this function into several smaller
/// ones to keep things simpler.
///
/// First and foremost the search itself can error due to
/// network issues.
///
/// The second point of failure is that the search might not
/// return any results.
///
/// Additional points of failure either fall into similar
/// categories as the ones above or are results of calling
/// `unwrap()` on the `extension()` and `to_str()` operands.
///
/// In other words this function returns a wide range of
/// errors which are not being logged or handled properly.
///
/// # TODO
///
/// Improve the error handling in this function
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

    // get the image list based on the game
    let images = client.get_images_for_id(first_game.id, &Grid(None)).await?;
    // client.get_official_steam_images(steam_app_id)

    // get get the file extensions in a scuffed manner
    let temp = PathBuf::from(&images[0].url);
    let extension = temp.extension().unwrap();
    info!("The file extension is: {:?}", &extension);
    let complete_path = path_out.join(first_game.name.to_owned() + "." + extension.to_str().unwrap());
    trace!("The complete path is: {:?}", complete_path);

    // get the image
    let response = reqwest::get(&images[0].url).await?;
    if response.status().is_success() {

        let mut dest = fs::File::create(complete_path)?;
        let bytes = response.bytes().await?;
        let mut content = bytes.as_ref();

        std::io::copy(&mut content, &mut dest)?;
        info!("Image saved as {}.{:?}", first_game.name, extension);
    } else {
        error!("Failed to fetch image: {}", response.status());
    }
    Ok(())
}

/// somehow will not override images for some reason
///
/// Optimizes images from one directory into another
///
/// This function takes in a file and a target
/// directory. It will  optimize and save it to
/// the `dir_out`. The images are resized to the target
/// dimension if provided, otherwise the dimensions are
/// preserverd.
///
/// # Errors
///
/// This function can only error with the following errors:
/// `NasError::FailedToReadFile`, `NasError::FailedToEncode`
/// or `NasError::FailedToWrite`. These errors are explanatory
pub fn optimize_image(file: &Path, dir_out: &Path, target_dimension: &Option<(u32, u32)>) -> Result<(), NasError> {

    let img = image::open(file).map_err(|e| {
        error!("Failed to read image at {:?} with {:?}", file, e);
        NasError::FailedToReadFile
    })?;
    
    let (w, h) = target_dimension.unwrap_or_else(|| img.dimensions());
    let size_factor = 1.0;

    let img: DynamicImage = image::DynamicImage::ImageRgba8(imageops::resize(
        &img,
        (w as f64 * size_factor) as u32,
        (h as f64 * size_factor) as u32,
        imageops::FilterType::Triangle,
    ));

    // webp encoder
    let encoder: Encoder = Encoder::from_image(&img).map_err(|e| {
        error!("Failed to encode the image at {:?} with {}", file, e);
        NasError::FailedToEncode
    })?;
    let webp: WebPMemory = encoder.encode(90f32); // quality as f32

    // let file_name = file.file_name().unwrap_or(std::ffi::OsStr::new("fail.webp"));
    // let file_name = file.file_stem().unwrap().join();
    let stem = file.file_stem().unwrap_or(std::ffi::OsStr::new("fail"));
    let file_name = std::ffi::OsString::from(format!("{}.webp", stem.to_string_lossy()));

    let out_path = dir_out.join(file_name);

    std::fs::write(&out_path, &*webp).map_err(|e| {
        error!("Failed to write imgage to file at {:?} with {:?}", out_path, e);
        NasError::FailedToWrite
    })?;
    Ok(())
}


/// Iterate over all images in one directory and output the
/// optimized images to another
///
/// This iterates over all files in a given directory `dir_in`,
/// filters them and then optimizes and saves them to a target
/// directory
/// 
/// # Errors
///
/// This
pub fn optimize_images(dir_in: &Path, dir_out: &Path) {
    // might error if empty
    let entries = match fs::read_dir(&dir_in) {
        Ok(entries) => entries,
        Err(e) => {
            error!("Failed to read input directory {:?} with {:?}", &dir_in, e);
            return;
        }
    };
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        match path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("png" | "jpg" | "webp") => {
                info!("File found at: {:?}", &dir_in);
                // ignore the errors
                let _ = optimize_image(&path, &dir_out, &Some((600,900)));
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
    // info!("Server settings path is {:?}", server_settings_path);
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
