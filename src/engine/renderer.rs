use crate::engine::types::*;
use crate::engine::shapes::*;

pub struct Renderer {
    width: i32,
    height: i32,
    tri: Vec<triangle::Triangle>
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Renderer {
        Renderer {
            width: width + 1,   // Add one for newlines
            height: height,
            tri: Vec::new()
        }
    }

    pub fn info(&self) {
        println!("Dimensions: {}, {}\nTriangles: {}", self.width, self.height, self.tri.len());
    }

    pub fn in_bounds(&self, c: &Coordinate) -> bool {
        c.x > 0.0 && c.y > 0.0 && c.x < (self.width as f64 - 1.0)&& c.y < (self.height as f64) // Subtract one to no overwrite \n
    }

    pub fn draw(&self) {
        // Initialize frame
        let mut frame: String = String::with_capacity((self.width * self.height + 1) as usize); // Add one for null terminator
        for i in 0..(self.width * self.height) {
            let mut c = ' ';
            if i == self.width * self.height {
                c = '\0'
            } else if i % self.width == self.width - 1 {
                c = '\n';
            }
            frame.insert(i as usize, c);
        }

        for t in self.tri.iter() {
            // (Debug Optional Draws)
            if cfg!(debug_assertions) {
                // Draw Triangle Vertices
                for i in 0..3 {
                    if self.in_bounds(&t.point[i]) {
                        //frame.remove((t.top_left.x as i32 + (t.top_left.y as i32 * self.width)) as usize);
                        frame.insert((t.point[i].x as i32 + (t.point[i].y as i32 * self.width)) as usize, '*');
                    }
                }

                // Draw Triangle Corners
                /*
                if self.in_bounds(&t.top_left) {
                    frame.insert((t.top_left.x as i32 + (t.top_left.y as i32 * self.width)) as usize, '#');
                }
                if self.in_bounds(&t.bot_right) {
                    frame.insert((t.bot_right.x as i32 + (t.bot_right.y as i32 * self.width)) as usize, '#');
                }
                */
            }
        }

        println!("{}", frame);
    }

    pub fn triangle(&mut self, points: [Coordinate; 3]) -> &triangle::Triangle {
        self.tri.push(triangle::Triangle::new(points));
        match self.tri.last() {
            Some(t) => t,
            None => panic!("Error allocation Triangle")
        }
    }
}