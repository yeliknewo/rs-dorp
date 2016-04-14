#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    
    pub fn new(position: [f32; 3]) -> Vertex {
        Vertex{
            position: position,
        }
    }
}


pub fn init_vertex() {
    implement_vertex!(Vertex, position);
}
