use crate::config::GlobalConfig;

use std::sync::Arc;

#[derive(PartialEq)]
pub enum Mode {
    Default,
    Portal,
}

#[derive(PartialEq)]
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
                "portal" => Mode::Portal,
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