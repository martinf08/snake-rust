use crate::food::Food;

use std::collections::LinkedList;
use piston_window::Key;


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
pub struct Segment {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct Snake {
    pub body: LinkedList<Segment>,
    direction: Direction,
    request_direction: Direction,
    pub just_eat: bool,
    pub next_head: Option<Segment>
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Segment> = LinkedList::new();

        for i in 0..3 {
            body.push_back(Segment { x: x - i, y });
        }

        Snake {
            body,
            direction: Direction::Right,
            request_direction: Direction::Right,
            just_eat: false,
            next_head: None
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn get_next_segment(&self) -> Segment {
        let (head_x, head_y) = self.head_position();

        match self.direction {
            Direction::Up => Segment { x: head_x, y: head_y - 1 },
            Direction::Down => Segment { x: head_x, y: head_y + 1 },
            Direction::Left => Segment { x: head_x - 1, y: head_y },
            Direction::Right => Segment { x: head_x + 1, y: head_y },
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

    pub fn overlap_tail(&self, x: &i32, y: &i32) -> bool {
        let mut index = 0;
        for segment in &self.body {
            if index > 0 {
                if (segment.x, segment.y) == (*x, *y) {

                    return true;
                }
            }
            index += 1;
        }

        false
    }

    pub fn update(&mut self) {
        if self.direction.opposite() != self.request_direction {
            self.direction = self.request_direction;
        }

        self.body.push_front(self.next_head.unwrap().clone());

        if self.just_eat {
            self.just_eat = false;

            return;
        }

        self.body.pop_back().unwrap();
    }

    pub fn next_move_eat(&self, food: &Food) -> bool {
        let Segment { x, y} = self.next_head.unwrap();

        return (x, y) == (food.x, food.y);
    }

    pub fn is_dead(&self, board_size: &f64, segment_size: &f64) -> bool {
        let (x, y) = self.head_position();
        let max_distance = *board_size as i32 / *segment_size as i32 - 1;

        self.overlap_tail(&x, &y) || x < 0 || x > max_distance || y < 0 || y > max_distance
    }
}