use crate::types::Coordinate;

pub fn area(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> f64 {
    ((a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)) / 2.0).abs()
}