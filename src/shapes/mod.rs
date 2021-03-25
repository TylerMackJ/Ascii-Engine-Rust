use crate::types::Coordinate;

pub fn circle(center: Coordinate, radius: usize, vertice_count: usize) -> Vec<Coordinate> {
    let mut points: Vec<Coordinate> = Vec::with_capacity(vertice_count);

    for i in 0..vertice_count {
        points.push(Coordinate{
            x: ((i as f64 / vertice_count as f64) * 2.0 * std::f64::consts::PI).sin() * radius as f64 + center.x,
            y: ((i as f64 / vertice_count as f64) * 2.0 * std::f64::consts::PI).cos() * radius as f64 + center.y,
        });
    }

    points
}