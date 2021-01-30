pub struct Score {
    pub title: String,
    pub best_score: u32,
    pub current_score: u32,
    pub retry_count: u32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            title: String::from("Scores"),
            best_score: 0,
            current_score: 0,
            retry_count: 0,
        }
    }
}