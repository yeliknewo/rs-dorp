use graphics::{RendererType, Window, SyncData, Renderers};
use components::renderables::{RenderableTex2, RenderableVertexColor, RenderableSolidColor};
use math::{Mat4};
use err::DorpErr;

#[derive(Debug, Clone)]
pub struct Renderable {
    renderer_type: RendererType,
    texture2d: Option<RenderableTex2>,
    vertex_color: Option<RenderableVertexColor>,
    solid_color: Option<RenderableSolidColor>,
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
        self.texture2d = Some(texture2d);
        self.renderer_type = RendererType::Texture2d;
    }


    pub fn set_vertex_color(&mut self, vertex_color: RenderableVertexColor) {
        self.vertex_color = Some(vertex_color);
        self.renderer_type = RendererType::VertexColor;
    }


    pub fn set_solid_color(&mut self, solid_color: RenderableSolidColor) {
        self.solid_color = Some(solid_color);
        self.renderer_type = RendererType::SolidColor;
    }


    pub fn get_renderer_type(&self) -> RendererType {
        self.renderer_type
    }


    pub fn get_texture2d(&self) -> Option<&RenderableTex2> {
        self.texture2d.as_ref()
    }


    pub fn get_solid_color(&self) -> Option<&RenderableSolidColor> {
        self.solid_color.as_ref()
    }


    pub fn get_vertex_color(&self) -> Option<&RenderableVertexColor> {
        self.vertex_color.as_ref()
    }


    pub fn get_mut_texture2d(&mut self) -> Option<&mut RenderableTex2> {
        self.texture2d.as_mut()
    }


    pub fn get_mut_solid_color(&mut self) -> Option<&mut RenderableSolidColor> {
        self.solid_color.as_mut()
    }


    pub fn get_mut_vertex_color(&mut self) -> Option<&mut RenderableVertexColor> {
        self.vertex_color.as_mut()
    }
}
