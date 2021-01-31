use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct GlobalConfig {
    pub computed_config: ComputedConfig
}

impl GlobalConfig {
    pub fn new() -> GlobalConfig {
        GlobalConfig {
            computed_config: ComputedConfig::new(Config::new())
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
    pub fn new(config: Config) -> ComputedConfig {
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
    game: Game,
}

#[derive(Deserialize)]
struct Game {
    event: String,
    wall: String,
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