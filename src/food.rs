use crate::snake::{Point};
use crate::board::Grid;

use std::collections::LinkedList;


#[derive(Copy, Clone)]
pub struct Food {
    pub x: i32,
    pub y: i32,
}

impl Food {
    pub fn new() -> Food {
        Food {
            x: 15,
            y: 15,
        }
    }

    pub fn get_food(
        &self,
        body: LinkedList<Point>,
        grid: Grid,
        food: &Food
    ) -> Option<Food> {

        let grid = grid.remove_occupied_positions(body, food);
        let (new_x, new_y) = grid.get_random_position();

        return Some(Food {
            x: new_x,
            y: new_y,
        });
    }
}