use ascii_engine::Renderer;
use ascii_engine::types::*;

fn main() {
    let mut r: Renderer = Renderer::new(20, 10);
    r.draw();
    
    let p: usize = r.polygon(vec![
        Coordinate{x: 0.0, y: 0.0}, 
        Coordinate{x: 5.0, y: 7.0}, 
        Coordinate{x: 20.0, y: 10.0},
        Coordinate{x: 10.0, y: 3.0}
    ]);
    
    r.draw();

    r.get_polygon(p).translate_vertex(3, -1.0, 1.0);

    r.draw();

    r.get_polygon(p).position_vertex(2, Coordinate {
        x: 10.0,
        y: 5.0
    });

    r.draw();


}
