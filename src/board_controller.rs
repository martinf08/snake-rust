use crate::board::Board;
use crate::food::Food;
use crate::score::Score;

use piston_window::{GenericEvent, Button};
use crossbeam_utils::thread;
use std::sync::{Arc, Mutex};


pub struct BoardController {
    pub board: Board,
    pub score: Score,
}

impl BoardController {
    pub fn new(board: Board, score: Score) -> BoardController {
        BoardController {
            board,
            score,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.board.snake.request_direction(key)
        }

        if let Some(args) = e.update_args() {
            if self.board.snake.is_dead(&*self.board.board_size, &*self.board.block_size) {
                self.score.reset();

                self.board = Board::new(
                    self.board.board_size.clone(),
                    self.board.block_size.clone(),
                    self.board.move_delay.clone(),
                    self.board.fps.clone(),
                )
            }

            self.board.current_delta += args.dt;

            if *self.board.move_delay <= self.board.current_delta {
                self.board.current_delta = 0.0;
            }

            self.board.snake.next_head = Some(self.board.snake.get_next_point());

            if self.board.snake.next_move_eat(&self.board.food) {
                self.board.food = self.get_next_food().unwrap();
                self.board.next_food = None;
                self.board.snake.just_eat = true;
                self.score.update_score();
            }

            self.board.snake.update(args.dt);
        }
    }

    pub fn get_next_food(&self) -> Option<Food> {
        let local_self = Arc::new(self);

        let new_food = Arc::new(Mutex::new(None));
        let new_food_clone = Arc::clone(&new_food);

        thread::scope(|s| {
            s.spawn(move |_| {
                *new_food_clone.lock().unwrap() = local_self.board.food.get_food(
                    local_self.board.snake.body.clone(),
                    local_self.board.grid.clone(),
                    &local_self.board.food,
                );
            });
        }).unwrap();

        return *new_food.lock().unwrap();
    }
}