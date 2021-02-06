use crate::board::Board;
use crate::food::Food;
use crate::game_mode::Mode;
use crate::portal::Portal;
use crate::score::Score;

use piston_window::{GenericEvent, Button};
use crossbeam_utils::thread;
use std::sync::{Arc, Mutex};


pub struct BoardController {
    pub board: Board,
    pub score: Score,
}

impl BoardController {
    pub fn new(mut board: Board, score: Score) -> BoardController {
        if board.game_mode.mode == Mode::Portal {
            let portal = Portal::new(&board);
            board.portal = Some(portal);
        }

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
            if self.board.snake.is_dead(&self.board.config.computed_config.board_size, &self.board.config.computed_config.block_size) {
                self.score.reset();

                self.board = Board::new(self.board.config.clone(), self.board.game_mode.clone());
            }

            self.board.current_delta += args.dt;

            if self.board.config.computed_config.move_delay <= self.board.current_delta {
                self.board.current_delta = 0.0;
            }

            self.board.snake.next_head = Some(
                self.board.snake.get_next_point(
                    &self.board.config.computed_config.board_size,
                    &self.board.config.computed_config.block_size,
                )
            );

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
                let grid = local_self
                    .board.grid
                    .clone()
                    .remove_occupied_positions(local_self.board.snake.body.clone(), &local_self.board.food, None);

                let (new_x, new_y) = grid.get_random_position();

                *new_food_clone.lock().unwrap() = Some(Food {
                    x: new_x,
                    y: new_y,
                });
            });
        }).unwrap();

        return *new_food.lock().unwrap();
    }
}