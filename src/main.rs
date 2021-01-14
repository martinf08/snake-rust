mod board;
mod board_controller;
mod board_view;
mod food;
mod snake;

use crate::board::Board;
use crate::board_controller::BoardController;
use crate::board_view::BoardView;

use piston_window::*;

fn main() {
    const BOARD_SIZE: f64 = 400.0;
    const SEGMENT_SIZE: f64 = 20.0;

    let mut window: PistonWindow = WindowSettings::new("snake-astar", [BOARD_SIZE, BOARD_SIZE])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut board = Board::new(BOARD_SIZE.clone(), SEGMENT_SIZE.clone());
    let mut board_controller = BoardController::new(board);

    let board_view = BoardView::new(BOARD_SIZE.clone(), SEGMENT_SIZE.clone());

    while let Some(event) = &window.next() {
        board_controller.event(event);

        if let Some(args) = event.render_args() {
            window.draw_2d(event, |context, graphics, _device| {
                board_view.draw(&board_controller, &context, graphics, &args)
            });
        }
    }
}