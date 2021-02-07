use crate::board_controller::BoardController;
use crate::score::Score;
use crate::snake::Snake;

use piston_window::types::Color;
use piston_window::*;
use std::sync::Arc;
use gfx_device_gl::Device;
use crate::config::GlobalConfig;
use crate::portal::Portal;


pub struct BoardViewSettings {
    block_size: Arc<f64>,
    snake_color: Color,
    board_background_color: Color,
    food_color: Color,
    gate_a_color: Color,
    gate_b_color: Color,
    gate_ellipse_x_ratio: f64,
}

impl BoardViewSettings {
    pub fn new(block_size: Arc<f64>) -> BoardViewSettings {
        BoardViewSettings {
            block_size,
            snake_color: [0.18, 0.80, 0.44, 1.0],
            board_background_color: [0.204, 0.286, 0.369, 1.0],
            food_color: [1.0, 0.0, 0.0, 1.0],
            gate_a_color: [0.043, 0.99, 0.97, 1.0],
            gate_b_color: [0.99, 0.98, 0.549, 1.0],
            gate_ellipse_x_ratio: 1.22,
        }
    }
}

pub struct GridViewSettings {
    board_size: Arc<f64>,
    block_size: Arc<f64>,
    grid_line_color: Color,
    line_radius: f64,
}

impl GridViewSettings {
    pub fn new(board_size: Arc<f64>, block_size: Arc<f64>) -> GridViewSettings {
        GridViewSettings {
            board_size,
            block_size,
            grid_line_color: [0.0, 0.0, 0.0, 0.8],
            line_radius: 1.0,
        }
    }
}

struct ScoreViewSettings {
    score_size: Arc<f64>,
    board_size: Arc<f64>,
    background_color: Color,
    title_color: Color,
    title_size: u32,
}

impl ScoreViewSettings {
    pub fn new(score_size: Arc<f64>, board_size: Arc<f64>) -> ScoreViewSettings {
        ScoreViewSettings {
            score_size,
            board_size,
            background_color: [0.0, 0.0, 0.0, 1.0],
            title_color: [1.0; 4],
            title_size: 22,
        }
    }
}

pub struct BoardView {
    board_settings: BoardViewSettings,
    grid_settings: GridViewSettings,
    score_settings: ScoreViewSettings,
    glyphs: Glyphs,
}

impl BoardView {
    pub fn new(config: Arc<GlobalConfig>, glyphs: Glyphs) -> BoardView {
        BoardView {
            board_settings: BoardViewSettings::new(Arc::new(config.computed_config.block_size)),
            grid_settings: GridViewSettings::new(
                Arc::new(config.computed_config.board_size),
                Arc::new(config.computed_config.block_size),
            ),
            score_settings: ScoreViewSettings::new(
                Arc::new(config.computed_config.score_size),
                Arc::new(config.computed_config.board_size),
            ),
            glyphs,
        }
    }

    pub fn draw(
        &mut self,
        controller: &mut BoardController,
        context: &Context,
        graphics: &mut G2d,
        device: &mut Device,
        _args: &RenderArgs,
    ) {
        clear(self.board_settings.board_background_color, graphics);

        self.draw_grid(context, graphics);
        self.draw_snake(&controller.board.snake, context, graphics);
        self.draw_scores(&controller.score, context, graphics, device);

        //Food
        self.draw_block(
            self.board_settings.food_color,
            *&controller.board.food.x,
            *&controller.board.food.y,
            context,
            graphics,
        );

        self.draw_gates(controller.board.portal.as_mut().unwrap(), context, graphics);
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
                graphics,
            );

            //Vertical
            line(
                self.grid_settings.grid_line_color,
                self.grid_settings.line_radius,
                [i, 0.0, i, *self.grid_settings.board_size],
                context.transform,
                graphics,
            );
        }
    }

    fn draw_snake(&self, snake: &Snake, context: &Context, graphics: &mut G2d) {
        for point in &snake.body {
            self.draw_block(self.board_settings.snake_color, point.x, point.y, &context, graphics)
        }
    }

    pub fn draw_block(&self, color: Color, x: f64, y: f64, context: &Context, graphics: &mut G2d) {
        let gui_x = x as f64 * *self.board_settings.block_size;
        let gui_y = y as f64 * *self.board_settings.block_size;

        rectangle(
            color,
            [gui_x, gui_y, *self.board_settings.block_size, *self.board_settings.block_size],
            context.transform,
            graphics,
        );
    }

    pub fn draw_ellipse(&self, color: Color, x: f64, y: f64, context: &Context, graphics: &mut G2d) {
        let gui_x = x as f64 * *self.board_settings.block_size;
        let gui_y = y as f64 * *self.board_settings.block_size;

        let ellipse_x_size = *self.board_settings.block_size / self.board_settings.gate_ellipse_x_ratio;
        let fixed_x = gui_x + ((*self.board_settings.block_size - ellipse_x_size) / 2.0);

        ellipse(
            color,
            [fixed_x, gui_y, ellipse_x_size, *self.board_settings.block_size],
            context.transform,
            graphics,
        );
    }

    pub fn draw_scores(&mut self, score: &Score, context: &Context, graphics: &mut G2d, device: &mut Device) {
        let end_x = *self.score_settings.board_size + *self.score_settings.score_size;

        rectangle(
            self.score_settings.background_color,
            [*self.score_settings.board_size,
                0.0,
                end_x,
                *self.score_settings.board_size
            ],
            context.transform,
            graphics,
        );

        text(
            self.score_settings.title_color,
            self.score_settings.title_size,
            &*score.title,
            &mut self.glyphs,
            context.transform.trans(
                &*self.score_settings.board_size + 20.0,
                40.0,
            ),
            graphics,
        ).unwrap();

        let mut i = 2.0;
        for (_k, mut score_element) in score.scores
            .clone()
            .into_iter() {
            text(
                self.score_settings.title_color,
                self.score_settings.title_size - 10,
                &*format!("{} : {}", ascii_uc_first(&mut score_element.title), &score_element.count),
                &mut self.glyphs,
                context.transform.trans(
                    &*self.score_settings.board_size + 10.0,
                    i * 60.0,
                ),
                graphics,
            ).unwrap();

            i += 1.0;
        }

        self.glyphs.factory.encoder.flush(device);
    }

    fn draw_gates(&self, portal: &mut Portal, context: &Context, graphics: &mut G2d) {
        for (i, gate) in portal.gates.iter().enumerate() {
            let color = match i {
                0 => Some(self.board_settings.gate_a_color),
                1 => Some(self.board_settings.gate_b_color),
                _ => None
            };

            let gate = gate.lock().unwrap();

            self.draw_ellipse(color.unwrap(), gate.x, gate.y, context, graphics);
        }
    }
}

fn ascii_uc_first(text: &mut str) -> &mut str {
    if let Some(char) = text.get_mut(0..1) {
        char.make_ascii_uppercase();
    }

    text
}