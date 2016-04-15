use std::sync::{Arc};
use std::collections::{HashMap};
use glium::Frame as GliumFrame;
use glium::texture::texture2d::{Texture2d};
use glium::texture::{RawImage2d};
use glium::{Surface, VertexBuffer, DrawParameters, IndexBuffer, Program};
use glium;
use image::{load_from_memory};

use logic::{Id};
use components::{Renderable};
use graphics::{Window, SyncData};
use graphics::texture2d::{Vertex, Index, DrawMethod, method_to_parameters, init_vertex};
use err::DorpErr;

#[derive(Debug)]
pub struct RendererTex2 {
    vertex_buffers: HashMap<Id, VertexBuffer<Vertex>>,
    index_buffers: HashMap<Id, IndexBuffer<Index>>,
    texture_buffers: HashMap<Id, Texture2d>,
    draw_parameters: HashMap<Id, DrawParameters<'static>>,
    program: Program,
}

impl RendererTex2 {
    pub fn new(window: &mut Window) -> Result<RendererTex2, DorpErr> {
        init_vertex();
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            in vec2 tex_coord;
            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            out vec2 v_tex_coord;

            void main() {
                v_tex_coord = tex_coord;
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coord;

            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex, v_tex_coord);
            }
        "#;
            Ok(
                RendererTex2 {
                vertex_buffers: HashMap::new(),
                index_buffers: HashMap::new(),
                texture_buffers: HashMap::new(),
                draw_parameters: HashMap::new(),
                program: match Program::from_source(window.get_facade(), vertex_shader_src, fragment_shader_src, None) {
                    Ok(program) => program,
                    Err(err) => return Err(DorpErr::GliumProgramCreation("Program From Source", err)),
                }
            }
        )
    }

    pub fn set_vertices(&mut self, id: Id, window: &mut Window, vertices: Vec<Vertex>) -> Result<(), DorpErr> {
        self.vertex_buffers.insert(id, match VertexBuffer::new(window.get_facade(), &vertices) {
            Ok(buffer) => buffer,
            Err(err) => return Err(DorpErr::GliumVertexBufferCreation("VertexBuffer New", err)),
        });
        Ok(())
    }

    pub fn set_indices(&mut self, id: Id, window: &mut Window, indices: Vec<Index>) -> Result<(), DorpErr> {
        self.index_buffers.insert(id, match IndexBuffer::new(window.get_facade(), glium::index::PrimitiveType::TrianglesList, &indices) {
            Ok(buffer) => buffer,
            Err(err) => return Err(DorpErr::GliumIndexBufferCreation("IndexBuffer New", err)),
        });
        Ok(())
    }

    pub fn set_texture(&mut self, id: Id, window: &mut Window, data: &[u8]) -> Result<(), DorpErr> {
        let texture = match load_from_memory(data) {
            Ok(texture) => texture,
            Err(err) => return Err(DorpErr::Image("Load From Memory data", err)),
        }.to_rgba();
        self.texture_buffers.insert(id, match Texture2d::new(window.get_facade(), RawImage2d::from_raw_rgba_reversed(texture.clone().into_raw(), texture.dimensions())) {
            Ok(texture) => texture,
            Err(err) => return Err(DorpErr::GliumTextureCreation("Texture2d New", err)),
        });
        Ok(())
    }

    pub fn set_draw_method(&mut self, id: Id, draw_method: DrawMethod) {
        self.draw_parameters.insert(id, method_to_parameters(draw_method));
    }

    pub fn render(&mut self, frame: &mut GliumFrame, renderable: Arc<Renderable>, sync_data: &SyncData) -> Result<(), DorpErr> {
        let renderable_tex2 = match renderable.get_texture2d() {
            Some(renderable) => renderable,
            None => return Err(DorpErr::Base("Renderable Get Tex2 was none")),
        };
        match frame.draw(
            match self.vertex_buffers.get(&renderable_tex2.get_vertex_id()) {
                Some(vertices) => vertices,
                None => return Err(DorpErr::Base("Self VertexBuffers Get was none")),
            },
            match self.index_buffers.get(&renderable_tex2.get_index_id()) {
                Some(indices) => indices,
                None => return Err(DorpErr::Base("Self index_buffers Get was none")),
            },
            &self.program,
            &uniform!(
                tex: match self.texture_buffers.get(&renderable_tex2.get_texture_id()) {
                    Some(texture) => texture,
                    None => return Err(DorpErr::Base("Self Texture Buffers Get was none")),
                },
                perspective: match sync_data.get_matrix(renderable_tex2.get_perspective_id()) {
                    Some(perspective) => *perspective,
                    None => return Err(DorpErr::Base("Matrix Data Get Matrix was none")),
                },
                view: match sync_data.get_matrix(renderable_tex2.get_view_id()) {
                    Some(view) => *view,
                    None => return Err(DorpErr::Base("Matrix Data Get Matrix was none")),
                },
                model: match sync_data.get_matrix(renderable_tex2.get_model_id()) {
                    Some(model) => *model,
                    None => return Err(DorpErr::Base("Matrix Data Get Matrix was none")),
                }
            ),
            match self.draw_parameters.get(&renderable_tex2.get_draw_method_id()) {
                Some(dp) => dp,
                None => return Err(DorpErr::Base("Self Draw parameters Get was none")),
            },
        ) {
            Ok(()) => (),
            Err(err) => return Err(DorpErr::GliumDraw("Frame Draw", err)),
        }
        Ok(())
    }
}
