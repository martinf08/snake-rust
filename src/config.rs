use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

pub struct GlobalConfig {
    pub computed_config: ComputedConfig,
    pub config: Arc<Config>,
}

impl GlobalConfig {
    pub fn new() -> GlobalConfig {
        let config = Arc::new(Config::new());
        GlobalConfig {
            computed_config: ComputedConfig::new(config.clone()),
            config: config.clone(),
        }
    }
}

pub struct ComputedConfig {
    pub board_size: f64,
    pub block_size: f64,
    pub move_delay: f64,
    pub score_size: f64,
    pub fps: f64,
}

impl ComputedConfig {
    pub fn new(config: Arc<Config>) -> ComputedConfig {
        ComputedConfig {
            board_size: (config.board.block_size * config.board.board_block_length) as f64,
            block_size: config.board.block_size as f64,
            move_delay: (config.game.level as f64 / 10.0),
            score_size: 150.0,
            fps: 60.0,
        }
    }
}

//Toml
#[derive(Deserialize)]
pub struct Config {
    pub board: Board,
    pub game: Game,
}

#[derive(Deserialize)]
pub struct Game {
    pub mode: String,
    pub wall: String,
    level: u32,
}

#[derive(Deserialize)]
pub struct Board {
    pub board_block_length: u32,
    pub block_size: u32,
}

impl Config {
    pub fn new() -> Config {
        let mut file = File::open(Path::new("./Config.toml")).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        toml::from_str(&*contents).unwrap()
    }
}