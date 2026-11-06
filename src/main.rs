mod board;
mod board_controller;
mod board_view;
mod config;
mod food;
mod game_mode;
mod portal;
mod score;
mod snake;

use crate::board::Board;
use crate::board_controller::BoardController;
use crate::board_view::BoardView;
use crate::config::GlobalConfig;
use crate::game_mode::GameMode;
use crate::score::Score;

use piston_window::*;
use std::path::Path;
use std::sync::Arc;

fn main() {
    let config = Arc::new(GlobalConfig::new());
    let mut window: PistonWindow = WindowSettings::new(
        "snake",
        [
            config.computed_config.board_size + config.computed_config.score_size,
            config.computed_config.board_size,
        ],
    )
    .exit_on_esc(true)
    .resizable(false)
    .build()
    .unwrap();

    let board = Board::new(
        config.clone(),
        Arc::new(GameMode::new(config.clone())),
        None,
    );

    let mut board_controller = BoardController::new(board, Score::new());

    let mut board_view = BoardView::new(
        config.clone(),
        window
            .load_font(Path::new("./assets/lcd-solid.ttf"))
            .unwrap(),
    );

    while let Some(event) = &window.next() {
        board_controller.event(event);

        if event.update_args().is_some() {
            window.set_title(
                if board_controller.won {
                    "Snake - board complete! R to restart"
                } else if board_controller.paused {
                    "Snake - paused (Space to resume)"
                } else {
                    "Snake - arrows to move | Space to pause | R to restart"
                }
                .into(),
            );
        }

        if let Some(args) = event.render_args() {
            // Piston already schedules render events at the window's frame rate.
            // Avoid a second timer here, which can skip alternating frames.
            window.draw_2d(event, |context, graphics, device| {
                board_view.draw(&mut board_controller, &context, graphics, device, &args)
            });
        }
    }
}

#[cfg(test)]
mod tests;
