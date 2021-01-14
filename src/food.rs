use crate::snake::Snake;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Food {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) exists: bool
}

impl Food {
    pub fn new() -> Food {
        Food {
            x: 0,
            y: 0,
            exists: false
        }
    }

    pub fn get_food(
        &self, food: &Food,
        snake: &Snake,
        board_size: &f64,
        segment_size: &f64
    ) -> Option<Food> {

        if !food.exists {
            while let (new_food_x, new_food_y) = self.generate_position(&board_size, &segment_size) {
                if snake.overlap_tail(&new_food_x, &new_food_y) {
                   continue
                }

                return Some(Food {
                    x: new_food_x,
                    y: new_food_y,
                    exists: true
                });
            }
        }
        None
    }

    pub fn generate_position(&self, board_size: &f64, segment_size: &f64) -> (i32, i32) {
        let mut rng = thread_rng();

        (
            rng.gen_range(1..*board_size as i32 / *segment_size as i32),
            rng.gen_range(1..*board_size as i32 / *segment_size as i32)
        )
    }

}