pub struct Food {
    pub position: (f64, f64, f64, f64),
}

impl Food {
    pub fn new() -> Self {
        Food {
            position: (300.0, 300.0, 15.0, 15.0),
        }
    }
}