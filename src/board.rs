use crate::config::GlobalConfig;
use crate::food::Food;
use crate::snake::{Snake, Point, FrameHandler};

use rand::seq::SliceRandom;
use std::iter::FromIterator;
use std::sync::Arc;
use std::collections::LinkedList;

pub struct Board {
    pub config: Arc<GlobalConfig>,
    pub snake: Snake,
    pub food: Food,
    pub next_food: Option<Food>,
    pub current_delta: f64,
    pub grid: Grid,
}

impl Board {
    pub fn new(config: Arc<GlobalConfig>) -> Board {

        Board {
            config: config.clone(),
            snake: Snake::new(
                4.0,
                4.0,
                FrameHandler::new(config.clone()),
            ),
            food: Food::new(),
            next_food: None,
            current_delta: 0.0,
            grid: Grid::new(
                *Arc::new(&config.computed_config.board_size),
                *Arc::new(&config.computed_config.block_size)
            )
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

        for x in 1..=(*board_size as i32 / *block_size as i32) - 1 {
            for y in 1..=(*board_size as i32 / *block_size as i32) - 1 {
                list.push((x as f64, y as f64))
            }
        }

        Grid { list }
    }

    pub fn remove_occupied_positions(mut self, body: LinkedList<Point>, food: &Food) -> Grid {
        let mut body = body.into_iter();

        while let Some(Point { x: body_x, y: body_y }) = &body.next() {
            self.list = Vec::from_iter(self.list.into_iter()
                .filter(|(x, y)| (*x, *y) != (*body_x, *body_y)));
        }

        self.list = Vec::from_iter(self.list.into_iter()
            .filter(|(x, y)| (*x, *y) != (food.x, food.y)));

        self
    }

    pub fn get_random_position(&self) -> (f64, f64) {
        return *self.list.choose(&mut rand::thread_rng()).unwrap();
    }
}