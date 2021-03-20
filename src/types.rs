pub struct Coordinate {
    pub x: f64,
    pub y: f64
}

impl Copy for Coordinate {}

impl Clone for Coordinate {
    fn clone(&self) -> Coordinate {
        *self
    }
}