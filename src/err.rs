use glium;
use image;
use std;

#[derive(Debug)]
pub enum DorpErr {
    BaseString(String),
    DorpString(String, Box<DorpErr>),
    GliumDrawString(String, glium::DrawError),
    GliumProgramCreationString(String, glium::program::ProgramCreationError),
    Base(&'static str),
    Dorp(&'static str, Box<DorpErr>),
    GliumDraw(&'static str, glium::DrawError),
    GliumProgramCreation(&'static str, glium::program::ProgramCreationError),
    GliumVertexBufferCreation(&'static str, glium::vertex::BufferCreationError),
    GliumIndexBufferCreation(&'static str, glium::index::BufferCreationError),
    GliumTextureCreation(&'static str, glium::texture::TextureCreationError),
    Image(&'static str, image::ImageError),
    Error(&'static str, Box<std::error::Error>),
    GliumSwapBuffers(&'static str, glium::SwapBuffersError),
    GliumCreation(&'static str, glium::GliumCreationError<glium::glutin::CreationError>),
}
