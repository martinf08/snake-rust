use crate::board::Board;
use crate::food::Food;

use piston_window::{GenericEvent, Button};
use crossbeam_utils::thread;
use std::sync::{Arc, Mutex};


pub struct BoardController {
    pub board: Board
}

impl BoardController {
    pub fn new(board: Board) -> BoardController {
        BoardController { board }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.board.snake.request_direction(key)
        }

        if let Some(args) = e.update_args() {
            if self.board.snake.is_dead(&self.board.board_size, &self.board.segment_size) {
                self.board = Board::new(self.board.board_size, self.board.segment_size)
            }

            self.board.current_delta += args.dt;

            if self.board.move_delay > self.board.current_delta {
                return;
            }

            self.board.current_delta = 0.0;

            self.board.snake.next_head = Some(self.board.snake.get_next_segment());

            if self.board.snake.next_move_eat(&self.board.food) {
                self.board.food = self.get_next_food().unwrap();
                self.board.next_food = None;
                self.board.snake.just_eat = true;
            }

            self.board.snake.update();
        }
    }

    pub fn get_next_food(&self) -> Option<Food> {
        let shared_self = Arc::new(&self);
        let new_food = Arc::new(Mutex::new(None));
        let new_food_clone = new_food.clone();

        thread::scope(|s| {
            s.spawn(move |_| {
                *new_food_clone.lock().unwrap() = shared_self.board.food.get_food(
                    shared_self.board.snake.clone(),
                    shared_self.board.grid.clone(),
                    &shared_self.board.food
                );
            });
        }).unwrap();

        return *new_food.lock().unwrap();
    }
}