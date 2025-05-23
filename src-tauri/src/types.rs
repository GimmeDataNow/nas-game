use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard};


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
    pub collection: Mutex<Vec<Game>>, // TODO: explore if a hashset is a better choice
}

impl GameLibrary {
    #[allow(dead_code)]
    pub fn new() -> Self { Self { collection: Mutex::new(Vec::new()) } }
}

