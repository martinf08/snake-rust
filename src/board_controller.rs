use crate::board::Board;
use crate::food::Food;
use crate::game_mode::Mode;
use crate::portal::Portal;
use crate::score::Score;
use piston_window::{Button, GenericEvent, Key};

pub struct BoardController {
    pub board: Board,
    pub score: Score,
    pub paused: bool,
    pub won: bool,
}

impl BoardController {
    pub fn new(mut board: Board, score: Score) -> BoardController {
        // Also support small configured boards where (15, 15) is outside the grid.
        if let Some(food) = Self::available_food(&board) {
            board.food = food;
        }
        if board.game_mode.mode == Mode::Portal {
            board.portal = Portal::new(&board);
        }
        BoardController { board, score, paused: false, won: false }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Space if !self.won => self.paused = !self.paused,
                Key::R => self.restart(),
                _ => self.board.snake.request_direction(key),
            }
        }
        if let Some(args) = e.update_args() {
            self.update(args.dt);
        }
    }

    pub fn restart(&mut self) {
        self.score.reset();
        let board = Board::new(self.board.config.clone(), self.board.game_mode.clone(), None);
        let score = std::mem::replace(&mut self.score, Score::new());
        *self = Self::new(board, score);
    }

    pub fn update(&mut self, dt: f64) {
        if self.paused || self.won || !dt.is_finite() || dt <= 0.0 {
            return;
        }
        let step = 1.0 / self.board.config.computed_config.fps;
        // Bound catch-up after a long stall while preserving ordinary frame remainders.
        self.board.current_delta += dt.min(0.25);
        while self.board.current_delta + f64::EPSILON >= step {
            self.board.current_delta -= step;
            self.tick(step);
            if self.won { break; }
        }
    }

    fn tick(&mut self, dt: f64) {
        self.board.snake.next_head = Some(self.board.snake.get_next_point(
            &self.board.config.computed_config.board_size,
            &self.board.config.computed_config.block_size,
        ));

        if self.board.snake.jump.is_some() && !self.board.snake.in_gate() {
            self.board.snake.jump = None;
            self.board.portal = Portal::new(&self.board);
        }
        if let Some(portal) = self.board.portal.as_mut() {
            if !portal.is_used() && self.board.snake.next_move_take_gate(portal) {
                self.board.snake.teleport(portal.clone());
            }
        }
        self.board.snake.update(dt);

        if self.board.snake.is_dead(
            &self.board.config.computed_config.board_size,
            &self.board.config.computed_config.block_size,
        ) {
            self.restart();
            return;
        }

        let (x, y) = self.board.snake.head_position();
        if (x.round(), y.round()) == (self.board.food.x, self.board.food.y) {
            self.board.snake.just_eat = true;
            self.score.update_score();
            if let Some(food) = Self::available_food(&self.board) {
                self.board.food = food;
            } else {
                // Remove optional gates before declaring the board complete.
                self.board.portal = None;
                if let Some(food) = Self::available_food(&self.board) {
                    self.board.food = food;
                } else {
                    self.won = true;
                }
            }
        }
    }

    fn available_food(board: &Board) -> Option<Food> {
        let gates = board.portal.as_ref().map(|portal| {
            portal.gates.iter().cloned().map(Some).collect()
        });
        board.grid.clone()
            .remove_occupied_positions(board.snake.body.clone(), &board.food, gates)
            .get_random_position()
            .map(|(x, y)| Food { x, y })
    }
}
