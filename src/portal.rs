use crate::board_controller::BoardController;

use crossbeam_utils::thread;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone)]
pub struct Gate {
    pub x: f64,
    pub y: f64,
    pub used: bool,
}

pub struct Portal {
    pub gate_a: Gate,
    pub gate_b: Gate,
}

impl Portal {
    pub fn new(controller: &BoardController) -> Portal {
        let local_self = Arc::new(controller);

        let gates: Arc<Mutex<Vec<Option<Gate>>>> = Arc::new(Mutex::new(Vec::new()));
        let gates_clone = Arc::clone(&gates);

        thread::scope(|s| {
            s.spawn(move |_| {
                while gates_clone.lock().unwrap().len() < 2 {
                    let grid = local_self
                        .board.grid
                        .clone()
                        .remove_occupied_positions(
                            local_self.board.snake.body.clone(),
                            &local_self.board.food,
                            Some(gates_clone.lock().unwrap().clone()),
                        );

                    if gates_clone.lock().unwrap().len() == 0 {
                        let (x, y) = grid.get_random_position();
                        gates_clone.lock().unwrap().push(Some(Gate { x, y, used: false }));

                        continue;
                    }

                    let (x, y) = grid.get_random_position();
                    gates_clone.lock().unwrap().push(Some(Gate { x, y, used: false }));
                }
            });
        }).unwrap();

        let gate_a = gates.lock().unwrap()[0].unwrap().clone();
        let gate_b = gates.lock().unwrap()[1].unwrap().clone();

        Portal { gate_a, gate_b }
    }
}