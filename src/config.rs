use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub struct GlobalConfig {
    pub computed_config: ComputedConfig,
    pub config: Arc<Config>,
}

impl GlobalConfig {
    pub fn new() -> GlobalConfig {
        Self::load(Path::new("./Config.toml")).unwrap_or_else(|error| {
            eprintln!("Configuration error: {}", error);
            std::process::exit(1);
        })
    }

    pub fn load(path: &Path) -> Result<GlobalConfig, String> {
        let contents =
            fs::read_to_string(path).map_err(|error| format!("{}: {}", path.display(), error))?;
        Self::from_toml(&contents)
    }

    pub fn from_toml(contents: &str) -> Result<GlobalConfig, String> {
        let config: Config = toml::from_str(contents).map_err(|error| error.to_string())?;
        config.validate()?;
        let config = Arc::new(config);
        Ok(GlobalConfig {
            computed_config: ComputedConfig::new(config.clone()),
            config: config.clone(),
        })
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
    fn validate(&self) -> Result<(), String> {
        if !(6..=256).contains(&self.board.board_block_length) {
            return Err("board_block_length must be between 6 and 256".into());
        }
        if self.board.block_size == 0 || self.board.block_size > 1024 {
            return Err("block_size must be between 1 and 1024".into());
        }
        if self.board.block_size * self.board.board_block_length > 8192 {
            return Err("board dimensions must not exceed 8192 pixels".into());
        }
        let distance = self.board.block_size as f64 * self.game.level as f64 / 600.0;
        if distance <= 0.0 || distance > 1.0 {
            return Err("level must produce a movement distance in (0, 1] per tick".into());
        }
        if !matches!(self.game.mode.as_str(), "default" | "portal") {
            return Err("mode must be 'default' or 'portal'".into());
        }
        if !matches!(self.game.wall.as_str(), "solid" | "fluid") {
            return Err("wall must be 'solid' or 'fluid'".into());
        }
        Ok(())
    }
}
