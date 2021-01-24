mod board;
mod board_controller;
mod board_view;
mod food;
mod snake;

use crate::board::Board;
use crate::board_controller::BoardController;
use crate::board_view::BoardView;

use piston_window::*;
use std::sync::Arc;


fn main() {
    let board_size = Arc::new(400.0);
    let block_size = Arc::new(20.0);
    let move_delay = Arc::new(0.14); //1 = board in one sec
    let fps = Arc::new(60.0);

    let mut window: PistonWindow = WindowSettings::new(
        "snake",
        [*Arc::clone(&board_size), *Arc::clone(&board_size)],
    ).exit_on_esc(true).resizable(false).build().unwrap();

    let board = Board::new(
        Arc::clone(&board_size),
        Arc::clone(&block_size),
        Arc::clone(&move_delay),
        Arc::clone(&fps),
    );

    let mut board_controller = BoardController::new(board);

    let board_view = BoardView::new(Arc::clone(&board_size), Arc::clone(&block_size));

    while let Some(event) = &window.next() {
        board_controller.event(event);

        if let Some(args) = event.render_args() {
            window.draw_2d(event, |context, graphics, _device| {
                board_view.draw(&board_controller, &context, graphics, &args)
            });
        }
    }
}