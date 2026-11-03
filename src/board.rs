use crate::config::GlobalConfig;
use crate::food::Food;
use crate::game_mode::GameMode;
use crate::portal::{Gate, Portal};
use crate::snake::{Snake, Point, FrameHandler};

use rand::seq::SliceRandom;
use std::collections::LinkedList;
use std::sync::Arc;


pub struct Board {
    pub config: Arc<GlobalConfig>,
    pub snake: Snake,
    pub food: Food,
    pub current_delta: f64,
    pub grid: Grid,
    pub game_mode: Arc<GameMode>,
    pub portal: Option<Portal>,
}

impl Board {
    pub fn new(config: Arc<GlobalConfig>, game_mode: Arc<GameMode>, portal: Option<Portal>) -> Board {
        Board {
            config: config.clone(),
            snake: Snake::new(
                4.0,
                4.0,
                FrameHandler::new(config.clone()),
                game_mode.clone(),
            ),
            food: Food::new(),
            current_delta: 0.0,
            grid: Grid::new(
                &config.computed_config.board_size,
                &config.computed_config.block_size,
            ),
            game_mode: game_mode.clone(),
            portal,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub list: Vec<(f64, f64)>
}

impl Grid {
    pub fn new(board_size: &f64, block_size: &f64) -> Grid {
        let mut list = Vec::new();

        for x in 0..(*board_size / *block_size) as i32 {
            for y in 0..(*board_size / *block_size) as i32 {
                list.push((x as f64, y as f64))
            }
        }

        Grid { list }
    }

    pub fn remove_occupied_positions(mut self, body: LinkedList<Point>, food: &Food, gates: Option<Vec<Option<Gate>>>) -> Grid {
        self.list.retain(|&(x, y)| {
            !body.iter().any(|point| (point.x.round(), point.y.round()) == (x, y))
                && (x, y) != (food.x, food.y)
                && !gates.as_ref().into_iter().flatten().flatten()
                    .any(|gate| (gate.x, gate.y) == (x, y))
        });

        self
    }

    pub fn get_random_position(&self) -> Option<(f64, f64)> {
        self.list.choose(&mut rand::thread_rng()).copied()
    }
}
