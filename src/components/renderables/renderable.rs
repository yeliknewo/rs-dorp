use std::sync::{Arc};

use graphics::{RendererType, Window, SyncData, Renderers};
use components::renderables::{RenderableTex2, RenderableVertexColor, RenderableSolidColor};
use math::{Mat4};
use err::DorpErr;

#[derive(Debug)]
pub struct Renderable {
    renderer_type: RendererType,
    texture2d: Option<Arc<RenderableTex2>>,
    vertex_color: Option<Arc<RenderableVertexColor>>,
    solid_color: Option<Arc<RenderableSolidColor>>,
}

impl Renderable {
    pub fn new() -> Renderable {
        Renderable {
            renderer_type: RendererType::Empty,
            texture2d: None,
            vertex_color: None,
            solid_color: None,
        }
    }

    pub fn new_from(other: Arc<Renderable>) -> Renderable {
        Renderable {
            renderer_type: other.renderer_type,
            texture2d: match other.texture2d.clone() {
                Some(tex2) => {
                    Some(Arc::new(RenderableTex2::new_from(tex2)))
                },
                None => None,
            },
            vertex_color: match other.vertex_color.clone() {
                Some(vertex) => {
                    Some(Arc::new(RenderableVertexColor::new_from(vertex)))
                },
                None => None,
            },
            solid_color: match other.solid_color.clone() {
                Some(solid) => {
                    Some(Arc::new(RenderableSolidColor::new_from(solid)))
                },
                None => None,
            }
        }
    }

    pub fn render(&mut self, window: &mut Window, sync_data: &mut SyncData, renderers: &mut Renderers) -> Result<(), DorpErr> {
        match self.get_renderer_type() {
            RendererType::Texture2d => match self.get_mut_texture2d() {
                Some(tex2) => match tex2.render(window, sync_data, renderers) {
                    Ok(()) => (),
                    Err(err) => return Err(DorpErr::Dorp("Tex2 Render", Box::new(err))),
                },
                None => (),
            },
            RendererType::SolidColor => match self.get_mut_solid_color() {
                Some(solid) => match solid.render(window, sync_data, renderers) {
                    Ok(()) => (),
                    Err(err) => return Err(DorpErr::Dorp("Solid Render", Box::new(err))),
                },
                None => (),
            },
            RendererType::VertexColor => match self.get_mut_vertex_color() {
                Some(vertex) => match vertex.render(window, sync_data, renderers) {
                    Ok(()) => (),
                    Err(err) => return Err(DorpErr::Dorp("Vertex Render", Box::new(err))),
                },
                None => (),
            },
            RendererType::Empty => return Err(DorpErr::Base("Self Get Renderer Type was Empty")),
        }
        Ok(())
    }

    pub fn set_model(&mut self, matrix: Mat4) -> Result<(), DorpErr> {
        match self.get_renderer_type() {
            RendererType::Texture2d => match self.get_mut_texture2d() {
                Some(tex2) => tex2.set_model(matrix),
                None => return Err(DorpErr::Base("Self Get Mut Texture2d was None")),
            },
            RendererType::SolidColor => match self.get_mut_solid_color() {
                Some(solid_color) => solid_color.set_model(matrix),
                None => return Err(DorpErr::Base("Self Get Mut Solid Color was None")),
            },
            RendererType::VertexColor => match self.get_mut_vertex_color() {
                Some(vertex_color) => vertex_color.set_model(matrix),
                None => return Err(DorpErr::Base("Self Get Mut Vertex Color was None")),
            },
            RendererType::Empty => return Err(DorpErr::Base("Self Get Renderer Type was Empty")),
        }
        Ok(())
    }


    pub fn set_texture2d(&mut self, texture2d: RenderableTex2) {
        self.texture2d = Some(Arc::new(texture2d));
        self.renderer_type = RendererType::Texture2d;
    }


    pub fn set_vertex_color(&mut self, vertex_color: RenderableVertexColor) {
        self.vertex_color = Some(Arc::new(vertex_color));
        self.renderer_type = RendererType::VertexColor;
    }


    pub fn set_solid_color(&mut self, solid_color: RenderableSolidColor) {
        self.solid_color = Some(Arc::new(solid_color));
        self.renderer_type = RendererType::SolidColor;
    }


    pub fn get_renderer_type(&self) -> RendererType {
        self.renderer_type
    }


    pub fn get_texture2d(&self) -> Option<Arc<RenderableTex2>> {
        self.texture2d.clone()
    }


    pub fn get_solid_color(&self) -> Option<Arc<RenderableSolidColor>> {
        self.solid_color.clone()
    }


    pub fn get_vertex_color(&self) -> Option<Arc<RenderableVertexColor>> {
        self.vertex_color.clone()
    }


    pub fn get_mut_texture2d(&mut self) -> Option<&mut RenderableTex2> {
        match self.texture2d.as_mut() {
            Some(texture2d) => Arc::get_mut(texture2d),
            None => None,
        }
    }


    pub fn get_mut_solid_color(&mut self) -> Option<&mut RenderableSolidColor> {
        match self.solid_color.as_mut() {
            Some(solid_color) => Arc::get_mut(solid_color),
            None => None,
        }
    }


    pub fn get_mut_vertex_color(&mut self) -> Option<&mut RenderableVertexColor> {
        match self.vertex_color.as_mut() {
            Some(vertex_color) => Arc::get_mut(vertex_color),
            None => None,
        }
    }
}
