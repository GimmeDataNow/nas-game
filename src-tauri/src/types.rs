//! This crate is for defining and implementing convenience
//! functions for types used throughout the program. 
use serde::{Serialize, Deserialize};
use std::sync::Mutex;


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
    pub fn new(launcher: Launcher, id: String) -> Self { Self { launcher, id } }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameLibrary {
    pub collection: Mutex<Vec<Game>>, // TODO: explore if a hashset is a better choice
}

impl GameLibrary {
    #[allow(dead_code)]
    pub fn new() -> Self { Self { collection: Mutex::new(Vec::new()) } }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameNameRequest {
    pub games: Vec<String>,
}

