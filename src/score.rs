use std::collections::{HashMap};

pub struct Score {
    pub title: String,
    pub scores: HashMap<String, ScoreElement>,
}

impl Score {
    pub fn new() -> Score {
        let mut scores: HashMap<String, ScoreElement> = HashMap::new();

        for title in vec!["current", "best", "death"].drain(..) {
            scores.insert(String::from(title), ScoreElement::new(String::from(title)));
        }

        Score {
            title: String::from("Scores"),
            scores,
        }
    }

    pub fn update_score(&mut self) {
        self.scores.get_mut("current").unwrap().count += 1;
        if self.scores.get("current").unwrap().count > self.scores.get("best").unwrap().count {
            self.scores.get_mut("best").unwrap().count = self.scores.get("current").unwrap().count;
        }
    }

    pub fn reset(&mut self) {
        self.scores.get_mut("death").unwrap().count += 1;
        self.scores.get_mut("current").unwrap().count = 0;
    }
}

#[derive(Clone)]
pub struct ScoreElement {
    pub title: String,
    pub count: u32,
}

impl ScoreElement {
    pub fn new(title: String) -> ScoreElement {
        ScoreElement {
            title,
            count: 0,
        }
    }
}