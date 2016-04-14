#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coord: [f32; 2],
}

impl Vertex {
    
    pub fn new(position: [f32; 3], tex_coord: [f32; 2]) -> Vertex {
        Vertex{
            position: position,
            tex_coord: tex_coord,
        }
    }
}


pub fn init_vertex() {
    implement_vertex!(Vertex, position, tex_coord);
}
