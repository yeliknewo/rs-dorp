use graphics::texture2d::{RendererTex2};
use graphics::solid_color::{RendererSolidColor};
use graphics::vertex_color::{RendererVertexColor};
use graphics::{Window};
use err::DorpErr;

#[derive(Debug)]
pub struct Renderers {
    renderer_solid_color: RendererSolidColor,
    renderer_vertex_color: RendererVertexColor,
    renderer_texture2d: RendererTex2,
}

impl Renderers {
    pub fn new(window: &mut Window) -> Result<Renderers, DorpErr> {
        Ok(
            Renderers {
                renderer_solid_color: match RendererSolidColor::new(window) {
                    Ok(solid) => solid,
                    Err(err) => return Err(DorpErr::Dorp("RendererSolidColor New", Box::new(err))),
                },
                renderer_vertex_color: match RendererVertexColor::new(window) {
                    Ok(vertex) => vertex,
                    Err(err) => return Err(DorpErr::Dorp("RendererVertexColor New", Box::new(err))),
                },
                renderer_texture2d: match RendererTex2::new(window) {
                    Ok(tex2) => tex2,
                    Err(err) => return Err(DorpErr::Dorp("RendererTex2 New", Box::new(err))),
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

#[derive(Debug, Copy, Clone)]
pub enum RendererType {
    SolidColor,
    VertexColor,
    Texture2d,
    Empty,
}
