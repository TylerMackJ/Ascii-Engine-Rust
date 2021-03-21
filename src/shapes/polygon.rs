use crate::shapes::*;

pub fn is_inside(vertices: &Vec<Coordinate>, point: Coordinate) -> bool {
    let horizontal: [Coordinate; 2] = [point, Coordinate {
        x: std::f64::MAX,
        y: point.y
    }];

    let mut intersections: i32 = 0;
    for i in 0..vertices.len() - 1 {
        let compare_line: [Coordinate; 2] = [vertices[i], vertices[i + 1]];
        println!("{} -> {}", point.x, i);
        if triangulation::is_intersecting(&horizontal, &compare_line) {
            intersections += 1;
        }
    }
    println!("{}", intersections);
    if intersections % 2 == 0 {
        false
    } else { 
        true
    }
}