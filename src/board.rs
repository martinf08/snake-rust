use crate::snake::Snake;
use crate::food::Food;

use piston_window::types::Color;
use std::collections::LinkedList;

pub struct Board {
    pub board_size: f64,
    pub segment_size: f64,
    pub snake: Snake,
    pub food: Food,
    pub move_delay: f64,
    pub current_delta: f64,
    pub grid: Grid

}

impl Board {
    pub fn new(board_size: f64, segment_size: f64) -> Board {
        Board {
            board_size,
            segment_size,
            snake: Snake::new(4, 4),
            food: Food::new(),
            move_delay: 0.2,
            current_delta: 0.0,
            grid: Grid::new(&board_size, &segment_size)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    list: LinkedList<(i32, i32)>
}

impl Grid {
    pub fn new(board_size: &f64, segment_size: &f64) -> Grid {
        let mut list: LinkedList<(i32, i32)> = LinkedList::new();

        for x in 1..=*board_size as i32 / *segment_size as i32 {
            for y in 1..=*board_size as i32 / *segment_size as i32 {
                list.push_back((x, y))
            }
        }

        Grid { list }
    }
}