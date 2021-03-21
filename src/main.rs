use ascii_engine::Renderer;
use ascii_engine::types::*;

use std::{thread, time};

fn main() {
    let mut r: Renderer = Renderer::new(20, 10);

    let p: usize = r.polygon(vec![
        Coordinate{x: 0.0, y: 0.0}, 
        Coordinate{x: 5.0, y: 7.0}, 
        Coordinate{x: 20.0, y: 10.0},
        Coordinate{x: 10.0, y: 3.0}
    ]);

    r.draw();

    for i in 0..2000 {
        r.get_polygon(p).rotate_around(Coordinate {
            x: 10.0,
            y: 5.0
        }, std::f64::consts::PI / 100.0);

        r.draw();

        thread::sleep(time::Duration::from_millis(10));
    }


}
