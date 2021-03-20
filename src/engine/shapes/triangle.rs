use crate::engine::types::Coordinate;

pub struct Triangle {
    pub point: [Coordinate; 3],
    pub top_left: Coordinate,
    pub bot_right: Coordinate
}

impl Triangle {
    pub fn new(points: [Coordinate; 3]) -> Triangle {
        let mut top_left = Coordinate {x: points[0].x, y: points[0].y};
        let mut bot_right = Coordinate {x: points[0].x, y: points[0].y};

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

    pub fn inside(&self, p: &Coordinate) -> bool {
        let a0: f64 = area(&self.point[0], &self.point[1], &self.point[2]);
        let a1: f64 = area(p, &self.point[1], &self.point[2]);
        let a2: f64 = area(&self.point[0], p, &self.point[2]);
        let a3: f64 = area(&self.point[0], &self.point[1], p);

        0.001 > a1 + a2 + a3 - a0
    }
}

pub fn area(a: &Coordinate, b: &Coordinate, c: &Coordinate) -> f64 {
    ((a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)) / 2.0).abs()
}