mod engine;

use engine::*;
use engine::renderer::*;
use engine::types::*;

fn main() {
    let mut r: Renderer = Renderer::new(20, 10);
    r.info();
    r.draw();
    r.triangle([
        Coordinate{x: 1.0, y: 1.0}, 
        Coordinate{x: 5.0, y: 7.0}, 
        Coordinate{x: 10.0, y: 4.0}
    ]);
    r.info();
    r.draw();
}
