use crate::config::GlobalConfig;

use std::sync::Arc;
use std::str;

pub enum Mode {
    Default,
    Portal,
}

pub enum Wall {
    Fluid,
    Solid,
}

pub struct GameMode {
    pub mode: Mode,
    pub wall: Wall,
}

impl GameMode {
    pub fn new(config: Arc<GlobalConfig>) -> GameMode {
        GameMode {
            mode: match &config.config.game.mode[..] {
                "default" => Mode::Default,
                "portail" => Mode::Portal,
                _ => Mode::Default
            },
            wall: match &config.config.game.wall[..] {
                "solid" => Wall::Solid,
                "fluid" => Wall::Fluid,
                _ => Wall::Solid
            },
        }
    }
}