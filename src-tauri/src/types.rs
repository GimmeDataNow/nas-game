//! This crate is for defining and implementing convenience
//! functions for types used throughout the program. 
use serde::{Serialize, Deserialize};
use std::sync::Mutex;


const DEFAULT_IP_ADDR: &str = "127.0.0.1";
const DEFAULT_IP_PORT: u16 = 53317;

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
    pub ip: String,
    pub port: u16,
}

impl Default for ServerSettings {
    fn default() -> Self { Self { ip: DEFAULT_IP_ADDR.to_owned(), port: DEFAULT_IP_PORT} }
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

