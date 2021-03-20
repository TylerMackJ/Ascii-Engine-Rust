mod engine;

use engine::*;
use engine::renderer::*;
use engine::types::*;

fn main() {
    let mut r: Renderer = Renderer::new(20, 10);
    r.draw();
    
    r.triangle([
        Coordinate{x: 0.0, y: 0.0}, 
        Coordinate{x: 5.0, y: 7.0}, 
        Coordinate{x: 20.0, y: 10.0}
    ]);
    
    r.triangle([
        Coordinate{x: 0.0, y: 0.0}, 
        Coordinate{x: 5.0, y: 7.0}, 
        Coordinate{x: 20.0, y: 0.0}
    ]);
    r.draw();
}
