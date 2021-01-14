use crate::board::Board;
use crate::food::Food;

use piston_window::{GenericEvent, Button};

pub struct BoardController {
    pub(crate) board: Board
}

impl BoardController {
    pub fn new(board: Board) -> BoardController {
        BoardController {
            board
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.board.snake.request_direction(key)
        }

        if let Some(args) = e.update_args() {
            self.board.current_delta += args.dt;

            if self.board.move_delay > self.board.current_delta {
                return;
            }

            self.board.current_delta = 0.0;

            self.board.snake.next_head = Some(self.board.snake.get_next_segment());

            if let Some(food) = self.board.food.get_food(
                &self.board.food,
                self.board.snake.clone(),
                self.board.grid.clone()
            ) {
                self.board.food = food;
            }

            if self.board.snake.next_move_eat(
                &self.board.snake.next_head,
                &self.board.food,
            ) {
                self.board.food.exists = false;
                self.board.snake.just_eat = true;
            }

            self.board.snake.update();
        }
    }
}