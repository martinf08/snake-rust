use crate::snake::{Snake, Segment};
use crate::board::Grid;

use rand::{thread_rng};
use rand::seq::SliceRandom;
use std::iter::FromIterator;


#[derive(Debug)]
pub struct Food {
    pub x: i32,
    pub y: i32,
    pub exists: bool
}

impl Food {
    pub fn new() -> Food {
        Food {
            x: 15,
            y: 15,
            exists: true
        }
    }

    pub fn get_food(
        &self,
        food: &Food,
        snake: Snake,
        grid: Grid
    ) -> Option<Food> {

        if !food.exists {
            let grid = self.remove_snake_position(grid, snake);
            let (new_x, new_y) = self.get_food_position(grid);

            return Some(Food {
                x: new_x,
                y: new_y,
                exists: true,
            });
        }
        None
    }

    pub fn remove_snake_position(&self, mut grid: Grid, snake: Snake) -> Grid {
        let mut virtual_snake = snake.clone();

        virtual_snake.body.push_front(Segment {
            x: virtual_snake.next_head.unwrap().x,
            y: virtual_snake.next_head.unwrap().y
        });

        let mut body = virtual_snake.body.into_iter();
        while let Some(Segment { x: body_x, y: body_y }) = &body.next() {
            body.next();

            grid.list = Vec::from_iter(grid.list.into_iter()
                .filter(|(x, y)| (*x, *y) != (*body_x, *body_y)));
        }

        grid
    }

    pub fn get_food_position(&self, grid: Grid) -> (i32, i32) {
        return *grid.list.choose(&mut rand::thread_rng()).unwrap()
    }
}