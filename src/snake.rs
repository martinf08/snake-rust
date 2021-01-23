use crate::food::Food;

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

#[derive(Clone)]
pub struct Snake {
    pub body: LinkedList<Point>,
    direction: Direction,
    request_direction: Direction,
    pub just_eat: bool,
    pub next_head: Option<Point>,
    pub frame_handler: FrameHandler,
}

impl Snake {
    pub fn new(x: f64, y: f64, frame_handler: FrameHandler) -> Snake {
        let mut body: LinkedList<Point> = LinkedList::new();

        for f in FloatIterator::new_with_step(0.0, 3.0, frame_handler.get_move_distance()) {
            body.push_back(Point { x: x - f, y });
        }

        Snake {
            body,
            direction: Direction::Right,
            request_direction: Direction::Right,
            just_eat: false,
            next_head: None,
            frame_handler,
        }
    }

    pub fn head_position(&self) -> (f64, f64) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn get_next_point(&self) -> Point {
        let (head_x, head_y) = self.head_position();

        let move_distance = self.frame_handler.get_move_distance();

        match self.direction {
            Direction::Up => Point { x: head_x, y: head_y - move_distance },
            Direction::Down => Point { x: head_x, y: head_y + move_distance },
            Direction::Left => Point { x: head_x - move_distance, y: head_y },
            Direction::Right => Point { x: head_x + move_distance, y: head_y },
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
            if index > 0 {
                if (point.x, point.y) == (*x, *y) {
                    return true;
                }
            }
            index += 1;
        }

        false
    }

    pub fn update(&mut self, frame_update: bool, delta_time: f64) {

        self.frame_handler.current_delta += delta_time;

        if self.frame_handler.current_delta < (1.0 / &*self.frame_handler.fps) {

           return;
        }

        self.frame_handler.current_delta = 0.0;
        if !frame_update {
            if self.direction.opposite() != self.request_direction {
                self.direction = self.request_direction;
            }
        }

        self.body.push_front(self.next_head.unwrap().clone());

        if self.just_eat {
            self.just_eat = false;

            return;
        }

        self.body.pop_back().unwrap();
    }

    pub fn next_move_eat(&self, food: &Food) -> bool {
        let Point { x, y } = self.next_head.unwrap();

        return (x, y) == (food.x as f64, food.y as f64);
    }

    pub fn is_dead(&self, board_size: &f64, block_size: &f64) -> bool {
        let (x, y) = self.head_position();
        let max_distance = *board_size as f64 / *block_size as f64 - 1.0;

        self.overlap_tail(&x, &y) || x < 0.0 || x > max_distance || y < 0.0 || y > max_distance
    }
}

#[derive(Clone)]
pub struct FrameHandler {
    pub(crate) fps: Arc<f64>,
    pub(crate) move_delay: Arc<f64>,
    block_size: Arc<f64>,
    current_delta: f64
}

impl FrameHandler {
    pub fn new(fps: Arc<f64>, move_delay: Arc<f64>, block_size: Arc<f64>) -> FrameHandler {
        FrameHandler {
            fps,
            move_delay,
            block_size,
            current_delta: 0.0
        }
    }

    pub fn get_move_distance(&self) -> f64 {
        &*self.fps * &*self.move_delay / &*self.fps
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
    pub fn new(start: f64, end: f64, steps: u64) -> Self {
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
        Self::new(start, end, steps)
    }

    fn at(&self, pos: u64) -> f64 {
        let f_pos = pos as f64 / self.steps as f64;
        (1. - f_pos) * self.start + f_pos * self.end
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