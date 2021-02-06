use crossbeam_utils::thread;
use std::sync::{Arc, Mutex};
use crate::board::Board;

#[derive(Copy, Clone)]
pub struct Gate {
    pub x: f64,
    pub y: f64,
    pub used: bool,
}

#[derive(Copy, Clone)]
pub struct Portal {
    pub gate_a: Gate,
    pub gate_b: Gate,
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

                    let (x, y) = grid.get_random_position();
                    gates_clone.lock().unwrap().push(Some(Gate { x, y, used: false }));
                }
            });
        }).unwrap();

        let gate_a = gates.lock().unwrap()[0].unwrap().clone();
        let gate_b = gates.lock().unwrap()[1].unwrap().clone();

        Portal { gate_a, gate_b, next: Some(0) }
    }
}

impl Iterator for Portal {
    type Item = Gate;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(n) => {
                self.next = Some(n + 1);

                match n {
                    0 => Some(self.gate_a),
                    1 => Some(self.gate_b),
                    _ => None
                }
            }
            None => {
                self.next = Some(0);

                None
            }
        }
    }
}