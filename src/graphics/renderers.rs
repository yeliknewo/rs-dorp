use std::fmt;
use std::error::Error;

use graphics::texture2d::{RendererTex2, RendererTex2Err};
use graphics::solid_color::{RendererSolidColor, RendererSolidColorErr};
use graphics::vertex_color::{RendererVertexColor, RendererVertexColorErr};
use graphics::{Window};

pub struct Renderers {
    renderer_solid_color: RendererSolidColor,
    renderer_vertex_color: RendererVertexColor,
    renderer_texture2d: RendererTex2,
}

impl Renderers {
    
    pub fn new(window: &mut Window) -> Result<Renderers, RenderersErr> {
        Ok(
            Renderers {
                renderer_solid_color: match RendererSolidColor::new(window) {
                    Ok(solid) => solid,
                    Err(err) => return Err(RenderersErr::RendererSolidColor("RendererSolidColor New", err)),
                },
                renderer_vertex_color: match RendererVertexColor::new(window) {
                    Ok(vertex) => vertex,
                    Err(err) => return Err(RenderersErr::RendererVertexColor("RendererVertexColor New", err)),
                },
                renderer_texture2d: match RendererTex2::new(window) {
                    Ok(tex2) => tex2,
                    Err(err) => return Err(RenderersErr::RendererTexture2d("RendererTex2 New", err)),
                },
            }
        )
    }

    
    pub fn get_mut_solid_color(&mut self) -> &mut RendererSolidColor {
        &mut self.renderer_solid_color
    }

    
    pub fn get_mut_vertex_color(&mut self) -> &mut RendererVertexColor {
        &mut self.renderer_vertex_color
    }

    
    pub fn get_mut_texture2d(&mut self) -> &mut RendererTex2 {
        &mut self.renderer_texture2d
    }
}

#[derive(Copy, Clone)]
pub enum RendererType {
    SolidColor,
    VertexColor,
    Texture2d,
    Empty,
}

#[derive(Debug)]
pub enum RenderersErr {
    RendererSolidColor(&'static str, RendererSolidColorErr),
    RendererVertexColor(&'static str, RendererVertexColorErr),
    RendererTexture2d(&'static str, RendererTex2Err),
}

impl fmt::Display for RenderersErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderersErr::RendererSolidColor(_, ref err) => err.fmt(f),
            RenderersErr::RendererVertexColor(_, ref err) => err.fmt(f),
            RenderersErr::RendererTexture2d(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RenderersErr {
    fn description(&self) -> &str {
        match *self {
            RenderersErr::RendererSolidColor(_, ref err) => err.description(),
            RenderersErr::RendererVertexColor(_, ref err) => err.description(),
            RenderersErr::RendererTexture2d(_, ref err) => err.description(),
        }
    }
}
