use crate::snake::Snake;
use crate::board_controller::BoardController;

use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d, RenderArgs, clear, line};
use std::sync::Arc;


pub struct BoardViewSettings {
    block_size: Arc<f64>,
    snake_color: Color,
    board_background_color: Color,
    food_color: Color
}

impl BoardViewSettings {
    pub fn new(block_size: Arc<f64>) -> BoardViewSettings {
        BoardViewSettings {
            block_size,
            snake_color: [0.18, 0.80, 0.44, 1.0],
            board_background_color: [0.204, 0.286, 0.369, 1.0],
            food_color: [1.0, 0.0, 0.0, 1.0]
        }
    }
}

pub struct GridViewSettings {
    board_size: Arc<f64>,
    block_size: Arc<f64>,
    grid_line_color: Color,
    line_radius: f64
}

impl GridViewSettings {
    pub fn new(board_size: Arc<f64>, block_size: Arc<f64>) -> GridViewSettings {
        GridViewSettings {
            board_size,
            block_size,
            grid_line_color: [0.0, 0.0, 0.0, 0.8],
            line_radius: 1.0
        }
    }
}

pub struct BoardView {
    board_settings: BoardViewSettings,
    grid_settings: GridViewSettings
}

impl BoardView {
    pub fn new(board_size: Arc<f64>, block_size: Arc<f64>) -> BoardView {
        BoardView {
            board_settings: BoardViewSettings::new(block_size.clone()),
            grid_settings: GridViewSettings::new(board_size.clone(), block_size.clone())
        }
    }

    pub fn draw(
        &self,
        controller: &BoardController,
        context: &Context,
        graphics: &mut G2d,
        _args: &RenderArgs
    ) {
        clear(self.board_settings.board_background_color, graphics);

        self.draw_grid(context, graphics);
        self.draw_snake(&controller.board.snake, context, graphics);

        //Food
        self.draw_block(
            self.board_settings.food_color,
            *&controller.board.food.x,
            *&controller.board.food.y,
            context,
            graphics
        )
    }

    fn draw_grid(&self, context: &Context, graphics: &mut G2d) {
        for i in (0..=*self.grid_settings.board_size as usize)
            .rev()
            .step_by(*self.grid_settings.block_size as usize) {
            let i = i as f64;

            //Horizontal
            line(
                self.grid_settings.grid_line_color,
                self.grid_settings.line_radius,
                [0.0, i, *self.grid_settings.board_size, i],
                context.transform,
                graphics
            );

            //Vertical
            line(
                self.grid_settings.grid_line_color,
                self.grid_settings.line_radius,
                [i, 0.0, i, *self.grid_settings.board_size],
                context.transform,
                graphics
            );
        }
    }

    fn draw_snake(&self, snake: &Snake, context: &Context, graphics: &mut G2d) {
        for point in &snake.body {
            self.draw_block(self.board_settings.snake_color, point.x, point.y, &context, graphics)
        }
    }

    pub fn draw_block(&self, color: Color, x: i32, y: i32, context: &Context, graphics: &mut G2d) {
        let gui_x = x as f64 * *self.board_settings.block_size;
        let gui_y = y as f64 * *self.board_settings.block_size;

        rectangle(
            color,
            [gui_x, gui_y, *self.board_settings.block_size, *self.board_settings.block_size],
            context.transform,
            graphics,
        );
    }
}