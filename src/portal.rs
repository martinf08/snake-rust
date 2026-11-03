use crate::board::Board;

#[derive(Clone)]
pub struct Gate {
    pub x: f64,
    pub y: f64,
    pub used: bool,
}

#[derive(Clone)]
pub struct Portal {
    pub gates: Vec<Gate>,
}

impl Portal {
    /// A nearly full board may not have room for two gates.
    pub fn new(board: &Board) -> Option<Portal> {
        let mut grid = board.grid.clone().remove_occupied_positions(
            board.snake.body.clone(),
            &board.food,
            None,
        );
        let (x, y) = grid.get_random_position()?;
        grid.list.retain(|&point| point != (x, y));
        let minimum_distance = board.config.config.board.board_block_length as f64 / 2.0;
        let mut distant = grid.clone();
        distant.list.retain(|&(other_x, other_y)| {
            (x - other_x).abs() + (y - other_y).abs() >= minimum_distance
        });
        let (other_x, other_y) = distant
            .get_random_position()
            .or_else(|| grid.get_random_position())?;
        Some(Portal {
            gates: vec![
                Gate { x, y, used: false },
                Gate {
                    x: other_x,
                    y: other_y,
                    used: false,
                },
            ],
        })
    }

    pub fn is_used(&self) -> bool {
        self.gates.iter().any(|gate| gate.used)
    }
}
