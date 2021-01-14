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

#[derive(Debug, Copy, Clone)]
pub struct Segment {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub struct Snake {
    pub(crate) body: LinkedList<Segment>,
    direction: Direction,
    request_direction: Direction,
    pub(crate) just_eat: bool,
    pub(crate) next_head: Option<Segment>
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
        if self.just_eat {
            if let Segment { x: next_x, y: next_y } = self.get_next_segment()  {
                let(head_x, head_y) = self.head_position();
                if (head_x, head_y) == (next_x, next_y) {
                    true;
                }
            }
        }
        for segment in &self.body {
            if (segment.x, segment.y) == (*x, *y) {
                true;
            }
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

    pub fn next_move_eat(&self, next_segment: &Option<Segment>, food: &Food) -> bool {
        let next_segment = next_segment.unwrap();
        return (next_segment.x, next_segment.y) == (food.x, food.y);
    }
}