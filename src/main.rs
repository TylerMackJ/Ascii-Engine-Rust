use ascii_engine::Renderer;
use ascii_engine::types::*;

fn main() {
    let mut r: Renderer = Renderer::new(20, 10);
    r.draw();
    
    r.polygon(vec![
        Coordinate{x: 0.0, y: 0.0}, 
        Coordinate{x: 5.0, y: 7.0}, 
        Coordinate{x: 20.0, y: 10.0},
        Coordinate{x: 10.0, y: 3.0}
    ]);
    
    r.draw();
}
