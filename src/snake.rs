use crate::config::GlobalConfig;
use crate::food::Food;
use crate::game_mode::{GameMode, Wall};
use crate::portal::{Portal, Gate};

use std::collections::LinkedList;
use piston_window::Key;
use std::sync::Arc;


#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone)]
pub struct Jump {
    gate_head: Option<Point>,
    pub is_jumping: bool,
    gate_end_position: Option<(f64, f64)>,
}

impl Jump {
    pub fn new(point: Point) -> Jump {
        Jump {
            gate_head: Some(point),
            is_jumping: true,
            gate_end_position: Some((point.x, point.y)),
        }
    }
}

#[derive(Clone)]
pub struct Snake {
    pub body: LinkedList<Point>,
    direction: Direction,
    request_direction: Direction,
    request_position: Option<(f64, f64)>,
    pub just_eat: bool,
    pub next_head: Option<Point>,
    pub frame_handler: FrameHandler,
    blocks_to_add: u32,
    middle_block_passed: bool,
    game_mode: Arc<GameMode>,
    pub jump: Option<Jump>,
}

impl Snake {
    pub fn new(x: f64, y: f64, frame_handler: FrameHandler, game_mode: Arc<GameMode>) -> Snake {
        let mut body: LinkedList<Point> = LinkedList::new();

        for f in FloatIterator::new_with_step(0.0, 2.0, frame_handler.get_move_distance()) {
            body.push_back(Point { x: x - f, y });
        }

        Snake {
            body,
            direction: Direction::Right,
            request_direction: Direction::Right,
            request_position: None,
            just_eat: false,
            next_head: None,
            frame_handler,
            blocks_to_add: 0,
            middle_block_passed: false,
            game_mode: game_mode.clone(),
            jump: None,
        }
    }

    pub fn head_position(&self) -> (f64, f64) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn get_next_point(&self, board_size: &f64, block_size: &f64) -> Point {
        let (head_x, head_y) = self.head_position();

        let move_distance = self.frame_handler.get_move_distance();
        let max_distance = *board_size as f64 / *block_size as f64 - 1.0;

        match self.game_mode.wall {
            Wall::Solid => match self.direction {
                Direction::Up => Point { x: head_x, y: head_y - move_distance },
                Direction::Down => Point { x: head_x, y: head_y + move_distance },
                Direction::Left => Point { x: head_x - move_distance, y: head_y },
                Direction::Right => Point { x: head_x + move_distance, y: head_y },
            },
            Wall::Fluid => match self.direction {
                Direction::Up => {
                    let next_y = if head_y - move_distance < 0.0 {
                        max_distance
                    } else {
                        head_y - move_distance
                    };

                    Point { x: head_x, y: next_y }
                }
                Direction::Down => {
                    let next_y = if head_y + move_distance > max_distance {
                        0.0
                    } else {
                        head_y + move_distance
                    };

                    Point { x: head_x, y: next_y }
                }
                Direction::Left => {
                    let next_x = if head_x - move_distance < 0.0 {
                        max_distance
                    } else {
                        head_x - move_distance
                    };

                    Point { x: next_x, y: head_y }
                }
                Direction::Right => {
                    let next_x = if head_x + move_distance > max_distance {
                        0.0
                    } else {
                        head_x + move_distance
                    };

                    Point { x: next_x, y: head_y }
                }
            }
        }
    }

    pub fn request_direction(&mut self, key: Key) {
        self.request_direction = match key {
            Key::Up => Direction::Up,
            Key::Down => Direction::Down,
            Key::Left => Direction::Left,
            Key::Right => Direction::Right,
            _ => self.request_direction.clone()
        };
    }

    pub fn overlap_tail(&self, x: &f64, y: &f64) -> bool {
        let mut index = 0;
        for point in &self.body {
            if index > ((1.0 / self.frame_handler.get_move_distance()) / 2.0).ceil() as u32 {
                if (point.x, point.y) == (*x, *y) {
                    return true;
                }
            }
            index += 1;
        }

        false
    }

    pub fn update(&mut self, delta_time: f64) {
        self.frame_handler.current_delta += delta_time;

        if self.frame_handler.current_delta < (1.0 / &self.frame_handler.config.computed_config.fps) {
            return;
        }

        self.frame_handler.current_delta = 0.0;

        match self.jump.as_mut() {
            Some(jump) => {
                if let Some(point) = jump.gate_head {
                    self.body.push_front(point);
                    jump.gate_head = None;
                    jump.is_jumping = true;
                    return;
                }
            }
            None => ()
        }


        let mut need_new_head = true;

        if self.direction.opposite() != self.request_direction {
            if let Some((past_head_x, past_head_y)) = self.request_position {
                let (head_x, head_y) = self.head_position();

                if head_x.round() != past_head_x.round() || head_y.round() != past_head_y.round() {
                    self.middle_block_passed = true
                }

                if self.at_ceil_edge((&head_x, &head_y)) && self.middle_block_passed {
                    self.middle_block_passed = false;
                    need_new_head = false;
                    self.request_position = None;

                    self.body.push_front(Point { x: head_x.round(), y: head_y.round() });
                    self.direction = self.request_direction;
                }
            }
        }

        if self.direction.opposite() != self.request_direction {
            self.request_position = Some(self.head_position());
        }

        if need_new_head {
            self.body.push_front(self.next_head.unwrap().clone());
        }

        if self.just_eat {
            self.blocks_to_add += (1.0 / self.frame_handler.get_move_distance()).ceil() as u32;
            self.just_eat = false;
        }

        if self.blocks_to_add > 0 {
            self.blocks_to_add -= 1;

            return;
        } else if self.blocks_to_add == 0 {
            self.body.pop_back().unwrap();
        }
    }

    pub fn next_move_eat(&self, food: &Food) -> bool {
        let Point { x, y } = self.next_head.unwrap();

        return (x.round(), y.round()) == (food.x, food.y);
    }

    pub fn next_move_take_gate(&self, portal: &mut Portal) -> bool {
        let Point { x, y } = self.next_head.unwrap();

        for gate in portal.gates.iter() {
            let mut gate = gate.lock().unwrap();
            if (x.round(), y.round()) != (gate.x, gate.y) {
                continue;
            }

            gate.used = true;

            return true;
        }

        false
    }

    pub fn teleport(&mut self, portal: Portal) {
        let Gate { x, y, .. } = *portal.gates
            .iter()
            .filter(|gate| !gate.lock().unwrap().used)
            .last()
            .unwrap()
            .lock()
            .unwrap();

        self.jump = Some(Jump::new(Point { x, y }));
    }

    pub fn is_dead(&self, board_size: &f64, block_size: &f64) -> bool {
        let (x, y) = self.head_position();
        let max_distance = *board_size as f64 / *block_size as f64 - 1.0;

        match self.game_mode.wall {
            Wall::Fluid => self.overlap_tail(&x, &y) && (x != max_distance && y != max_distance),
            Wall::Solid => self.overlap_tail(&x, &y) || x < 0.0 || x > max_distance || y < 0.0 || y > max_distance
        }
    }

    pub fn in_gate(&mut self) -> bool {
        let Point { x: tail_x, y: tail_y } = self.body.back().unwrap();
        let (gate_x, gate_y) = self.jump.unwrap().gate_end_position.unwrap();

        if self.jump.unwrap().is_jumping && (tail_x, tail_y) == (&gate_x, &gate_y) {
            return false;
        }

        true
    }

    fn at_ceil_edge(&self, (head_x, head_y): (&f64, &f64)) -> bool {
        match self.direction {
            Direction::Up => (head_y - self.frame_handler.get_move_distance()).trunc() != head_y.trunc()
                || (head_y > &0.0 && head_y <= &self.frame_handler.get_move_distance()),
            Direction::Down => (head_y + self.frame_handler.get_move_distance()).trunc() != head_y.trunc(),
            Direction::Left => (head_x - self.frame_handler.get_move_distance()).trunc() != head_x.trunc()
                || (head_x > &0.0 && head_x <= &self.frame_handler.get_move_distance()),
            Direction::Right => (head_x + self.frame_handler.get_move_distance()).trunc() != head_x.trunc(),
        }
    }
}

#[derive(Clone)]
pub struct FrameHandler {
    pub config: Arc<GlobalConfig>,
    current_delta: f64,
}

impl FrameHandler {
    pub fn new(config: Arc<GlobalConfig>) -> FrameHandler {
        FrameHandler {
            config,
            current_delta: 0.0,
        }
    }

    pub fn get_move_distance(&self) -> f64 {
        (self.config.computed_config.block_size / &self.config.computed_config.fps) * &self.config.computed_config.move_delay
    }
}

pub struct FloatIterator {
    current: u64,
    current_back: u64,
    steps: u64,
    start: f64,
    end: f64,
}

impl FloatIterator {
    pub fn new(start: f64, end: f64, steps: u64) -> FloatIterator {
        FloatIterator {
            current: 0,
            current_back: steps,
            steps,
            start,
            end,
        }
    }

    pub fn new_with_step(start: f64, end: f64, step: f64) -> FloatIterator {
        let steps = ((end - start) / step).abs().round() as u64;
        FloatIterator::new(start, end, steps)
    }

    fn at(&self, pos: u64) -> f64 {
        let f_pos = pos as f64 / self.steps as f64;
        (1.0 - f_pos) * self.start + f_pos * self.end
    }
}

impl Iterator for FloatIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.current_back {
            return None;
        }
        let result = self.at(self.current);
        self.current += 1;
        Some(result)
    }
}