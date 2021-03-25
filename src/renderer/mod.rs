use crate::*;
use crate::core::*;

pub struct Renderer {
    width: usize,
    height: usize,
    width_scale: f64,
    pub frame_count: usize,
    poly: Vec<Polygon>
}

impl Renderer {
    pub fn new(width: usize, height: usize, width_scale: f64) -> Renderer {
        Renderer {
            width: width + 1,   // Add one for newlines
            height: height,
            width_scale: width_scale,
            frame_count: 0,
            poly: Vec::new()
        }
    }

    fn info(&self) {
        let mut tri_count: usize = 0;
        for p in self.poly.iter() {
            tri_count += p.tri.len();
        }
        println!("Dimensions: {}, {}\nFrame: {}\nPolys: {} | Tris: {}", self.width, self.height, self.frame_count, self.poly.len(), tri_count);
    }

    fn in_bounds(&self, c: &Coordinate) -> bool {
        c.x >= 0.0 && c.y >= 0.0 && c.x < (self.width as f64 - 1.0) && c.y < (self.height as f64) // Subtract one to no overwrite \n
    }

    pub fn draw(&mut self) {
        // Initialize frame
        let mut frame: String = String::with_capacity((self.width * self.height + 1) as usize); // Add one for null terminator
        for i in 0..(self.width * self.height) {
            let mut c = ' ';
            if i == self.width * self.height {
                c = '\0'
            } else if i % self.width == self.width - 1 {
                c = '\n';
            }
            frame.push(c);
        }

        for p in self.poly.iter() {
            for t in p.tri.iter() {
                for x in std::cmp::max((t.top_left.x * self.width_scale) as usize, 0)..std::cmp::min((t.bot_right.x * self.width_scale + 1.0) as usize, self.width - 1) {
                    for y in std::cmp::max(t.top_left.y as usize, 0)..std::cmp::min((t.bot_right.y + 1.0) as usize, self.height) {
                        if t.is_inside(&Coordinate{x: x as f64 / self.width_scale, y: y as f64}) {
                            frame.insert((x as f64 as usize + (y * self.width)) as usize, '#');
                            frame.remove((x as f64 as usize + (y * self.width) + 1) as usize);
                        }
                    }
                }

                // (Debug Optional Draws)
                if cfg!(debug_assertions) {
                    // Draw Triangle Verticesi32
                    for i in 0..3 {
                        if self.in_bounds(&t.point[i]) {
                            //frame.remove((t.top_left.x as i32 + (t.top_left.y as i32 * self.width)) as usize);
                            frame.insert((t.point[i].x * self.width_scale) as usize + (t.point[i].y as usize * self.width), '0');
                            frame.remove((t.point[i].x * self.width_scale) as usize + (t.point[i].y as usize * self.width) + 1);
                        }
                    }

                    // Draw Triangle Corners
                    /*
                    if self.in_bounds(&t.top_left) {
                        frame.insert((t.top_left.x as i32 + (t.top_left.y as i32 * self.width)) as usize, '*');
                        frame.remove((t.top_left.x as i32 + (t.top_left.y as i32 * self.width) + 1) as usize);
                    }
                    if self.in_bounds(&t.bot_right) {
                        frame.insert((t.bot_right.x as i32 + (t.bot_right.y as i32 * self.width)) as usize, '*');
                        frame.remove((t.bot_right.x as i32 + (t.bot_right.y as i32 * self.width) + 1) as usize);
                    }
                    */
                }
            }
        }

        self.frame_count += 1;
        if cfg!(debug_assertions) {
            self.info();
        }
        println!("{}", frame);
    }

    pub fn polygon(&mut self, points: Vec<Coordinate>) -> usize {
        if points.len() < 3 {
            panic!("Error: Tried to create a polygon with less than 3 points");
        }
        self.poly.push(Polygon::new(points));
        self.poly.len() - 1
    }

    pub fn get(&mut self, polygon: usize) -> &mut Polygon {
        match self.poly.get_mut(polygon) {
            Some(p) => p,
            None => panic!("Polygon not found")
        }
    }
}