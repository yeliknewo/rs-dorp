use std::sync::{Arc};
use std::error::Error;
use std::fmt;
use std::collections::{HashMap};
use glium::Frame as GliumFrame;
use glium::{Surface, DrawError, VertexBuffer, IndexBuffer, DrawParameters, Program, ProgramCreationError};
use glium;

use components::{Renderable};
use logic::{Id};
use graphics::{SyncData, Window};
use graphics::solid_color::{Vertex, init_vertex, Index, DrawMethod, method_to_parameters};

pub struct RendererSolidColor {
    vertex_buffers: HashMap<Id, VertexBuffer<Vertex>>,
    index_buffers: HashMap<Id, IndexBuffer<Index>>,
    draw_parameters: HashMap<Id, DrawParameters<'static>>,
    program: Program,
}

impl RendererSolidColor {
    
    pub fn new(window: &mut Window) -> Result<RendererSolidColor, RendererSolidColorErr> {
        init_vertex();
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;

            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            void main() {
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            uniform vec4 u_color;

            void main() {
                color = u_color;
            }
        "#;
        Ok(
            RendererSolidColor {
                vertex_buffers: HashMap::new(),
                index_buffers: HashMap::new(),
                draw_parameters: HashMap::new(),
                program: match Program::from_source(window.get_facade(), vertex_shader_src, fragment_shader_src, None) {
                    Ok(program) => program,
                    Err(err) => return Err(RendererSolidColorErr::ProgramCreation("Program From Source", err)),
                },
            }
        )
    }

    
    pub fn set_vertices(&mut self, id: Id, window: &mut Window, vertices: Vec<Vertex>) -> Result<(), RendererSolidColorErr> {
        self.vertex_buffers.insert(id, match VertexBuffer::new(window.get_facade(), &vertices) {
            Ok(buffer) => buffer,
            Err(err) => return Err(RendererSolidColorErr::VertexBufferCreation("VertexBuffer New", err)),
        });
        Ok(())
    }

    
    pub fn set_indices(&mut self, id: Id, window: &mut Window, indices: Vec<Index>) -> Result<(), RendererSolidColorErr> {
        self.index_buffers.insert(id, match IndexBuffer::new(window.get_facade(), glium::index::PrimitiveType::TrianglesList, &indices) {
            Ok(buffer) => buffer,
            Err(err) => return Err(RendererSolidColorErr::IndexBufferCreation("IndexBuffer New", err)),
        });
        Ok(())
    }

    
    pub fn set_draw_method(&mut self, id: Id, draw_method: DrawMethod) {
        self.draw_parameters.insert(id, method_to_parameters(draw_method));
    }

    pub fn render(&mut self, frame: &mut GliumFrame, renderable: Arc<Renderable>, sync_data: &SyncData) -> Result<(), RendererSolidColorErr> {
        let renderable_solid = match renderable.get_solid_color() {
            Some(renderable) => renderable,
            None => return Err(RendererSolidColorErr::Get("Renderable Get Solid Color")),
        };
        match frame.draw(
            match self.vertex_buffers.get(&renderable_solid.get_vertex_id()) {
                Some(vertices) => vertices,
                None => return Err(RendererSolidColorErr::Get("Self Vertex Buffers Get")),
            },
            match self.index_buffers.get(&renderable_solid.get_index_id()) {
                Some(indices) => indices,
                None => return Err(RendererSolidColorErr::Get("Self Index Buffers Get")),
            },
            &self.program,
            &uniform!(
                u_color: match sync_data.get_vec4(renderable_solid.get_color_id()) {
                    Some(color) => *color,
                    None => return Err(RendererSolidColorErr::Get("Sync Data Get Vec4")),
                },
                perspective: match sync_data.get_matrix(renderable_solid.get_perspective_id()) {
                    Some(perspective) => *perspective,
                    None => return Err(RendererSolidColorErr::Get("Matrix Data Get Matrix")),
                },
                view: match sync_data.get_matrix(renderable_solid.get_view_id()) {
                    Some(view) => *view,
                    None => return Err(RendererSolidColorErr::Get("Matrix Data Get Matrix")),
                },
                model: match sync_data.get_matrix(renderable_solid.get_model_id()) {
                    Some(model) => *model,
                    None => return Err(RendererSolidColorErr::Get("Matrix Data Get Matrix")),
                }
            ),
            match self.draw_parameters.get(&renderable_solid.get_draw_method_id()) {
                Some(dp) => dp,
                None => return Err(RendererSolidColorErr::Get("Self Draw Parameters Get")),
            }
        ) {
            Ok(()) => (),
            Err(err) => return Err(RendererSolidColorErr::Draw("Frame Draw", err)),
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum RendererSolidColorErr {
    Get(&'static str),
    Draw(&'static str, DrawError),
    ProgramCreation(&'static str, ProgramCreationError),
    VertexBufferCreation(&'static str, glium::vertex::BufferCreationError),
    IndexBufferCreation(&'static str, glium::index::BufferCreationError),
}

impl fmt::Display for RendererSolidColorErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RendererSolidColorErr::Get(_) => write!(f, "Get was None"),
            RendererSolidColorErr::Draw(_, ref err) => err.fmt(f),
            RendererSolidColorErr::ProgramCreation(_, ref err) => err.fmt(f),
            RendererSolidColorErr::VertexBufferCreation(_, ref err) => err.fmt(f),
            RendererSolidColorErr::IndexBufferCreation(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RendererSolidColorErr {
    fn description(&self) -> &str {
        match *self {
            RendererSolidColorErr::Get(_) => "Get was None",
            RendererSolidColorErr::Draw(_, ref err) => err.description(),
            RendererSolidColorErr::ProgramCreation(_, ref err) => err.description(),
            RendererSolidColorErr::VertexBufferCreation(_, ref err) => err.description(),
            RendererSolidColorErr::IndexBufferCreation(_, ref err) => err.description(),
        }
    }
}
