mod board;
mod board_controller;
mod board_view;
mod config;
mod food;
mod score;
mod snake;

use crate::board::Board;
use crate::board_controller::BoardController;
use crate::board_view::BoardView;
use crate::config::GlobalConfig;
use crate::score::Score;

use piston_window::*;
use std::sync::Arc;
use std::path::Path;


fn main() {
    let config = Arc::new(GlobalConfig::new());

    let mut window: PistonWindow = WindowSettings::new(
        "snake",
        [
            config.computed_config.board_size + config.computed_config.score_size,
            config.computed_config.board_size
        ],
    ).exit_on_esc(true).resizable(false).build().unwrap();

    let board = Board::new(config.clone());

    let mut board_controller = BoardController::new(board, Score::new());

    let mut board_view = BoardView::new(
        config.clone(),
        window.load_font(Path::new("./assets/lcd-solid.ttf")).unwrap(),
    );

    let mut frame_delta = 0.0;
    while let Some(event) = &window.next() {
        board_controller.event(event);

        if let Some(args) = event.render_args() {
            frame_delta += args.ext_dt;
            if frame_delta < 1.0 / &config.computed_config.fps {
                continue;
            }

            window.draw_2d(event, |context, graphics, device| {
                board_view.draw(&board_controller, &context, graphics, device, &args)
            });
        }
    }
}