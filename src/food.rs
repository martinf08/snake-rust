#[derive(Copy, Clone)]
pub struct Food {
    pub x: f64,
    pub y: f64,
}

impl Food {
    pub fn new() -> Food {
        Food {
            x: 15.0,
            y: 15.0,
        }
    }
}