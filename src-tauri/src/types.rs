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

/// This struct represents one instance of a launcher and its game_id.
///
/// As already stated this struct represents one instance of a game in regards
/// to its game id. Meaning that one game can exist under several distinct
/// launchers at once. This struct will eventually be needed to launch the
/// game.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Launcher {
    /// The launcher's name
    pub name: String,
    /// The game id that is associated with this game and launcher
    pub game_id: String,
}

impl Launcher {
    pub fn new(launcher: String, game_id: String) -> Self { Self { name: launcher, game_id } }
}

/// This struct represents a game and its many potential launchers.
///
/// This struct contains both a `Vec` of all the launchers and a
/// `steam_grid_id`. Since the `Vec` can be empty this also allows for games to
/// be added to the library without having a valid launcher, making such
/// entries effectively just place holders. The `steam_grid_id` is used to
/// reteive details about the game from the steam grid api.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Game {
    launcher: Vec<Launcher>,
    /// The steam_grid_db id.
    ///
    /// # Note
    ///
    /// This is NOT the steam id of the game.
    steam_grid_id: Option<String>,
}

impl Game {
    /// Initialize a new instance without any data.
    pub fn new() -> Self { Self { launcher: Vec::new(), steam_grid_id: None } }
    pub fn set_launcher(&mut self, launcher: Launcher) {
        if !self.launcher.contains(&launcher) { self.launcher.push(launcher); }
    }
    pub fn set_steam_grid_id(&mut self, id: Option<String>) {
        self.steam_grid_id = id;
    }
}

/// This struct represents all of the games the server has saved.
///
/// Since this contais a `Vec` this means that the game libarary can be empty.
#[derive(Debug, Deserialize, Serialize)]
pub struct GameLibrary {
    pub collection: Vec<Game>, // TODO: explore if a hashset is a better choice
}

impl Default for GameLibrary {
    fn default() -> Self { Self { collection: Vec::new() } }
}

impl GameLibrary {
    #[allow(dead_code)]
    pub fn new() -> Self { Self { collection: Vec::new() } }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameNameRequest {
    pub games: Vec<String>,
}

