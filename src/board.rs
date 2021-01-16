use crate::snake::{Snake, Segment};
use crate::food::Food;

use std::iter::FromIterator;
use rand::seq::SliceRandom;


pub struct Board {
    pub board_size: f64,
    pub segment_size: f64,
    pub snake: Snake,
    pub food: Food,
    pub next_food: Option<Food>,
    pub move_delay: f64,
    pub current_delta: f64,
    pub grid: Grid,
}

impl Board {
    pub fn new(board_size: f64, segment_size: f64) -> Board {
        Board {
            board_size,
            segment_size,
            snake: Snake::new(4, 4),
            food: Food::new(),
            next_food: None,
            move_delay: 0.05,
            current_delta: 0.0,
            grid: Grid::new(&board_size, &segment_size),
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub list: Vec<(i32, i32)>
}

impl Grid {
    pub fn new(board_size: &f64, segment_size: &f64) -> Grid {
        let mut list= Vec::new();

        for x in 1..=(*board_size as i32 / *segment_size as i32) - 1 {
            for y in 1..=(*board_size as i32 / *segment_size as i32) - 1 {
                list.push((x, y))
            }
        }

        Grid { list }
    }

    pub fn remove_snake_position(mut self, snake: Snake) -> Grid {
        let mut body = snake.body.into_iter();

        while let Some(Segment { x: body_x, y: body_y }) = &body.next() {

            self.list = Vec::from_iter(self.list.into_iter()
                .filter(|(x, y)| (*x, *y) != (*body_x, *body_y)));
        }

        self
    }

    pub fn get_random_position(&self) -> (i32, i32) {
        return *self.list.choose(&mut rand::thread_rng()).unwrap()
    }
}