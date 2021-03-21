use ascii_engine::Renderer;
use ascii_engine::types::*;

use std::{thread, time};

fn main() {
    let mut r: Renderer = Renderer::new(40, 20);

    let p: usize = r.polygon(vec![
        Coordinate{x: 0.0, y: 0.0}, 
        Coordinate{x: 5.0, y: 7.0}, 
        Coordinate{x: 20.0, y: 10.0},
        Coordinate{x: 10.0, y: 3.0}
    ]);

    r.draw();

    for i in 0..2000 {
        let center: Coordinate = r.get_polygon(p).get_center();
        r.get_polygon(p).rotate_around(center, std::f64::consts::PI / 200.0);

        r.get_polygon(p).translate((i as f64 / 100.0).sin() * 0.1, (i as f64 / 100.0).cos() * 0.1);

        r.draw();

        thread::sleep(time::Duration::from_millis(10));
    }


}
