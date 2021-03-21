use crate::types::*;

pub fn is_intersecting(line1: &[Coordinate; 2], line2: &[Coordinate; 2]) -> bool {
    let dir1: i8 = direction(&line1[0], &line1[1], &line2[0]);
    let dir3: i8 = direction(&line2[0], &line2[1], &line1[0]);
    let dir2: i8 = direction(&line1[0], &line1[1], &line2[1]);
    let dir4: i8 = direction(&line2[0], &line2[1], &line1[1]);

    if dir1 != dir2 && dir3 != dir4 {
        return true;
    } else if dir1 == 0 && on_line(line1, &line2[0]) {
        return true;
    } else if dir2 == 0 && on_line(line1, &line2[1]) {
        return true;
    } else if dir3 == 0 && on_line(line2, &line1[0]) {
        return true;
    } else if dir4 == 0 && on_line(line2, &line1[1]) {
        return true;
    }
    false
}

fn direction(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> i8 {
    let val: f64 = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);
    
    if val.abs() < 0.000001 {
        0   // colinear
    } else if val < 0.0 {
        2   // anti-clockwise
    } else {
        1   // clockwise
    }
}

fn on_line(line1: &[Coordinate; 2], p: &Coordinate) -> bool {
    if  p.x <= line1[0].x.max(line1[1].x) &&
        p.x <= line1[0].x.max(line1[1].x) &&
        p.y <= line1[0].y.max(line1[1].y) &&
        p.y <= line1[0].y.max(line1[1].y) {
        return true;
    }
    false
}