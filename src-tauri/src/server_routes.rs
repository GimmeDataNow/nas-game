#[allow(unused_imports)]
use crate::{trace, info, warn, error, logging::LoggingLevel, logging::logging_function};
#[allow(unused_imports)]
use crate::types::{Launcher, Game, GameLibrary, GameNameRequest};
use crate::server::{default_cwd, optimize_images,fetch_image};

use std::fs;
use std::sync::Mutex;
use std::path::{Path, PathBuf};
use actix_web::{get, post, web, HttpResponse, Responder, http::StatusCode};
use serde_json;
use futures::stream::{self, StreamExt};

#[get("/")]
async fn route_hello() -> impl Responder {
    HttpResponse::Ok().body("Is Alive")
}
#[post("/echo")]
async fn route_echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[get("/add_dummy")]
async fn route_add_dummy_get(data: web::Data<Mutex<GameLibrary>>) -> impl Responder {
    let lib = &mut data.lock().unwrap().collection;
    lib.push(Game::new());
    format!("{:?}", lib)
}

#[post("/games")]
async fn route_add_to_games(data: web::Data<Mutex<GameLibrary>>, games: web::Json<Vec<Game>>) -> impl Responder {
    let lib = &mut data.lock().unwrap().collection;
    let mut counter = 0;
    // if I were to rewirte this for loop with the filter() method then it would
    // allow for duplicate entries to be made.
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
async fn route_save_library(filelocation: web::Data<PathBuf>, data: web::Data<Mutex<GameLibrary>>) -> impl Responder {
    let lib = &mut match data.lock() {
        Ok(s) => s.collection.clone(),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to aquire lock on game library")
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

fn image_exists(dir: &Path, name: &str) -> bool {
    let extensions = ["webp", "jpg", "jpeg", "png"];
    extensions.iter().any(|ext| { dir.join(format!("{}.{}", name, ext)).exists() })
}

#[post("/download_images")]
async fn route_download_images(data: web::Json<GameNameRequest>) -> impl Responder {

    let path = default_cwd().join("images").join("non-optimized");
    let missing_games: Vec<_> = data.games.iter()
        .filter(|game| !image_exists(&path, game))
        .cloned()
        .collect();
    info!("The images for these games will be fetched: {:?}", missing_games);

    stream::iter(missing_games)
        .for_each_concurrent(Some(5), |game_name| {
            let path = path.clone();
            async move {
                if let Err(e) = fetch_image(&game_name, &path).await {
                    error!("Failed to fetch image for {}: {}", game_name, e);
                }
            }
        }).await;
    HttpResponse::build(StatusCode::OK).body("Images have been downloaded")
}

#[post("/optimize_images_server")]
async fn route_optimize_images_server() -> impl Responder {
    info!("Attempting to optimize images");
    let dir_in = default_cwd().join("images").join("non-optimized");
    let dir_out = default_cwd().join("images").join("optimized");
    optimize_images(&dir_in, &dir_out);
    HttpResponse::build(StatusCode::OK).body("library has been saved")
}
