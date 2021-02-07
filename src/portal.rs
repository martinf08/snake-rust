use crate::board::Board;

use crossbeam_utils::thread;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Gate {
    pub x: f64,
    pub y: f64,
    pub used: bool,
}

#[derive(Clone)]
pub struct Portal {
    pub gates: Vec<Arc<Mutex<Gate>>>,
    pub next: Option<u32>,
}

impl Portal {
    pub fn new(board: &Board) -> Portal {
        let local_self = Arc::new(board);

        let gates: Arc<Mutex<Vec<Option<Gate>>>> = Arc::new(Mutex::new(Vec::new()));
        let gates_clone = Arc::clone(&gates);

        thread::scope(|s| {
            s.spawn(move |_| {
                while gates_clone.lock().unwrap().len() < 2 {
                    let grid = local_self
                        .grid
                        .clone()
                        .remove_occupied_positions(
                            local_self.snake.body.clone(),
                            &local_self.food,
                            Some(gates_clone.lock().unwrap().clone()),
                        );

                    if gates_clone.lock().unwrap().len() == 0 {
                        let (x, y) = grid.get_random_position();
                        gates_clone.lock().unwrap().push(Some(Gate { x, y, used: false }));

                        continue;
                    }

                    let other_gates_clone = Arc::clone(&gates_clone);
                    let gates = other_gates_clone.lock().unwrap().last().cloned();
                    let last = gates.as_ref().unwrap().as_ref().unwrap();

                    'inner: for i in 0..3 {
                        if other_gates_clone.lock().unwrap().len() == 2 {
                            break 'inner;
                        }

                        let (new_x, new_y) = grid.get_random_position();

                        if i == 2 {
                            other_gates_clone.lock().unwrap().push(Some(Gate { x: new_x, y: new_y, used: false }));
                            break 'inner;
                        }
                        
                        if (last.x.abs() + last.y.abs()) - (new_x.abs() + new_y.abs()) < (board.config.config.board.board_block_length as f64 / 2.0).abs().ceil() {
                            continue;
                        }

                        other_gates_clone.lock().unwrap().push(Some(Gate { x: new_x, y: new_y, used: false }));
                    }
                }
                
            });
        }).unwrap();

        Portal {
            gates: gates
                .clone()
                .lock()
                .unwrap()
                .drain(..)
                .map(|gate|Arc::new(Mutex::new(gate.unwrap().clone())))
                .collect(),
            next: Some(0),
        }
    }

    pub fn is_used(&self, mut portal: Option<Portal>) -> bool {
        portal
            .as_mut()
            .unwrap()
            .gates
            .iter()
            .filter(|gate| gate.lock().unwrap().used)
            .count() > 0
    }
}