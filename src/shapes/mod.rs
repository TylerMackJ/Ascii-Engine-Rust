pub mod triangle;

use crate::types::*;

pub struct Triangle {
    pub point: [Coordinate; 3],
    pub top_left: Coordinate,
    pub bot_right: Coordinate
}

impl Triangle {
    pub fn new(points: [Coordinate; 3]) -> Triangle {
        let mut top_left = points[0];
        let mut bot_right = points[0];

        for i in 1..3 {
            if points[i].x < top_left.x {
                top_left.x = points[i].x;
            } else if points[i].x > bot_right.x {
                bot_right.x = points[i].x
            }

            if points[i].y < top_left.y {
                top_left.y = points[i].y
            } else if points[i].y > bot_right.y {
                bot_right.y = points[i].y;
            }
        }

        Triangle {
            point: points,
            top_left: top_left,
            bot_right: bot_right
        }
    }

    pub fn position_vertex(&mut self, vertex: usize, position: Coordinate) {
        self.point[vertex] = position;

        self.top_left = self.point[0];
        self.bot_right = self.point[0];

        for i in 1..3 {
            if self.point[i].x < self.top_left.x {
                self.top_left.x = self.point[i].x;
            } else if self.point[i].x > self.bot_right.x {
                self.bot_right.x = self.point[i].x
            }

            if self.point[i].y < self.top_left.y {
                self.top_left.y = self.point[i].y
            } else if self.point[i].y > self.bot_right.y {
                self.bot_right.y = self.point[i].y;
            }
        }
    }

    pub fn rotate_vertex(&mut self, vertex: usize, rotation_point: Coordinate, radians: f64) {
        let x: f64 = self.point[vertex].x - rotation_point.x;
        let y: f64 = self.point[vertex].y - rotation_point.y;

        self.position_vertex(vertex, Coordinate {
            x: (x * radians.cos()) - (y * radians.sin()) + rotation_point.x,
            y: (x * radians.sin()) + (y * radians.cos()) + rotation_point.y
        });
    }

    pub fn inside(&self, p: &Coordinate) -> bool {
        let a0: f64 = triangle::area(&self.point[0], &self.point[1], &self.point[2]);
        let a1: f64 = triangle::area(p, &self.point[1], &self.point[2]);
        let a2: f64 = triangle::area(&self.point[0], p, &self.point[2]);
        let a3: f64 = triangle::area(&self.point[0], &self.point[1], p);

        0.0000000001 > a1 + a2 + a3 - a0
    }
}

pub struct Polygon {
    pub tri: Vec<Triangle>
}

impl Polygon {
    pub fn new(points: Vec<Coordinate>) -> Polygon {
        let mut tri: Vec<Triangle> = Vec::new();
        for i in 1..(points.len() - 1) {
            tri.push(Triangle::new([points[0], points[i], points[i + 1]]))
        }
        Polygon {
            tri: tri
        }
    }

    pub fn translate_vertex(&mut self, vertex: usize, x_offset: f64, y_offset: f64) {
        if vertex == 0 {
            for t in self.tri.iter_mut() {
                t.position_vertex(0, Coordinate{
                    x: t.point[0].x + x_offset,
                    y: t.point[0].y + y_offset
                })
            }
        } else {
            // Check if moving last triangle of polygon
            if vertex - 1 != self.tri.len() {
                let tri1_option = self.tri.get_mut(vertex - 1);
                let tri1: &mut Triangle;
                match tri1_option {
                    Some(t) => tri1 = t,
                    None => panic!("Error getting last triangle")
                }
                tri1.position_vertex(1, Coordinate {
                    x: tri1.point[1].x + x_offset,
                    y: tri1.point[1].y + y_offset
                })
            }

            // Check if moving first triangle of polygon
            if vertex != 1 {
                let tri2_option = self.tri.get_mut(vertex - 2);
                let tri2: &mut Triangle;
                match tri2_option {
                    Some(t) => tri2 = t,
                    None => panic!("Error getting first triangle")
                }
                tri2.position_vertex(2, Coordinate {
                    x: tri2.point[2].x + x_offset,
                    y: tri2.point[2].y + y_offset
                })
            }
        }
    }

    pub fn position_vertex(&mut self, vertex: usize, position: Coordinate) {
        if vertex == 0 {
            for t in self.tri.iter_mut() {
                t.position_vertex(0, position);
            }
        } else {
            // Check if moving last triangle of polygon
            if vertex - 1 != self.tri.len() {
                let tri1_option = self.tri.get_mut(vertex - 1);
                let tri1: &mut Triangle;
                match tri1_option {
                    Some(t) => tri1 = t,
                    None => panic!("Error getting last triangle")
                }
                tri1.position_vertex(1, position);
            }

            // Check if moving first triangle of polygon
            if vertex != 1 {
                let tri2_option = self.tri.get_mut(vertex - 2);
                let tri2: &mut Triangle;
                match tri2_option {
                    Some(t) => tri2 = t,
                    None => panic!("Error getting first triangle")
                }
                tri2.position_vertex(2, position);
            }
        }
    }

    pub fn rotate_around(&mut self, rotation_point: Coordinate, radians: f64) {
        for t in self.tri.iter_mut() {
            for i in 0..3 {
                t.rotate_vertex(i, rotation_point, radians);
            }
        }
    }
}