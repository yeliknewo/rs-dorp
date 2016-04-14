#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    
    pub fn new(position: [f32; 3], color: [f32; 4]) -> Vertex {
        Vertex{
            position: position,
            color: color,
        }
    }
}


pub fn init_vertex() {
    implement_vertex!(Vertex, position, color);
}
