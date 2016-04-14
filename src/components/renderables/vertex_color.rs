use std::sync::{Arc};
use std::fmt;
use std::error::Error;

use logic::{Id, IdManager, IdType};
use graphics::{Window, SyncData, Renderers};
use graphics::vertex_color::{Vertex, Index, DrawMethod, RendererVertexColorErr};
use math::{Mat4};

struct Changes {
    vertices: Option<Vec<Vertex>>,
    indices: Option<Vec<Index>>,
    draw_method: Option<DrawMethod>,
    perspective: Option<(Mat4, Mat4)>,
    view: Option<(Mat4, Mat4)>,
    model: Option<(Mat4, Mat4)>,
    dirty_render: bool,
}

impl Changes {
    
    fn new() -> Changes {
        Changes {
            vertices: None,
            indices: None,
            draw_method: None,
            perspective: None,
            view: None,
            model: None,
            dirty_render: false,
        }
    }

    
    fn new_from(other: &Changes) -> Changes {
        Changes {
            vertices: other.vertices.clone(),
            indices: other.indices.clone(),
            draw_method: other.draw_method.clone(),
            perspective: other.perspective,
            view: other.view,
            model: other.model,
            dirty_render: other.dirty_render,
        }
    }
}

pub struct RenderableVertexColor {
    vertex_id: Id,
    index_id: Id,
    draw_method_id: Id,
    perspective_id: Id,
    view_id: Id,
    model_id: Id,
    changes: Changes,
}

impl RenderableVertexColor {
    
    pub fn new(manager: &mut IdManager) -> RenderableVertexColor {
        RenderableVertexColor {
            vertex_id: Id::new(manager, IdType::Vertex),
            index_id: Id::new(manager, IdType::Index),
            draw_method_id: Id::new(manager, IdType::DrawMethod),
            perspective_id: Id::new(manager, IdType::Matrix),
            view_id: Id::new(manager, IdType::Matrix),
            model_id: Id::new(manager, IdType::Matrix),
            changes: Changes::new(),
        }
    }

    
    pub fn new_from(other: Arc<RenderableVertexColor>) -> RenderableVertexColor {
        RenderableVertexColor {
            vertex_id: other.vertex_id,
            index_id: other.index_id,
            draw_method_id: other.draw_method_id,
            perspective_id: other.perspective_id,
            view_id: other.view_id,
            model_id: other.model_id,
            changes: Changes::new_from(&other.changes),
        }
    }

    pub fn render(&mut self, window: &mut Window, sync_data: &mut SyncData, renderers: &mut Renderers) -> Result<(), RenderableVertexColorErr> {
        if self.changes.dirty_render {
            match self.changes.vertices.clone() {
                Some(vertices) => match renderers.get_mut_vertex_color().set_vertices(self.vertex_id, window, vertices) {
                    Ok(()) => (),
                    Err(err) => return Err(RenderableVertexColorErr::RendererVertexColor("Renderers Get Mut Solid Color Set Vertices", err)),
                },
                None => (),
            }
            match self.changes.indices.clone() {
                Some(indices) => match renderers.get_mut_vertex_color().set_indices(self.index_id, window, indices) {
                    Ok(()) => (),
                    Err(err) => return Err(RenderableVertexColorErr::RendererVertexColor("Renderers Get Mut Solid Color Set Indices", err)),
                },
                None => (),
            }
            match self.changes.draw_method.clone() {
                Some(draw_method) => renderers.get_mut_vertex_color().set_draw_method(self.draw_method_id, draw_method),
                None => (),
            }
            match self.changes.perspective {
                Some(perspective) => sync_data.set_matrix(self.perspective_id, perspective.0, perspective.1),
                None => (),
            }
            match self.changes.view {
                Some(view) => sync_data.set_matrix(self.view_id, view.0, view.1),
                None => (),
            }
            match self.changes.model {
                Some(model) => sync_data.set_matrix(self.model_id, model.0, model.1),
                None => (),
            }
            self.changes.vertices = None;
            self.changes.indices = None;
            self.changes.draw_method = None;
            self.changes.perspective = None;
            self.changes.view = None;
            self.changes.model = None;
            self.changes.dirty_render = false;
        }
        Ok(())
    }

    
    pub fn set_vertices(&mut self, vertices: Vec<Vertex>) {
        self.changes.vertices = Some(vertices);
        self.changes.dirty_render = true;
    }

    
    pub fn set_indices(&mut self, indices: Vec<Index>) {
        self.changes.indices = Some(indices);
        self.changes.dirty_render = true;
    }

    
    pub fn set_draw_method(&mut self, draw_method: DrawMethod) {
        self.changes.draw_method = Some(draw_method);
        self.changes.dirty_render = true;
    }

    
    pub fn set_perspective(&mut self, matrix: Mat4) {
        self.changes.perspective = Some((matrix, matrix.to_inverse()));
        self.changes.dirty_render = true;
    }

    
    pub fn set_view(&mut self, matrix: Mat4) {
        self.changes.view = Some((matrix, matrix.to_inverse()));
        self.changes.dirty_render = true;
    }

    
    pub fn set_model(&mut self, matrix: Mat4) {
        self.changes.model = Some((matrix, matrix.to_inverse()));
        self.changes.dirty_render = true;
    }

    
    pub fn set_vertex_id(&mut self, id: Id) {
        self.vertex_id = id;
    }

    
    pub fn set_index_id(&mut self, id: Id) {
        self.index_id = id;
    }

    
    pub fn set_draw_method_id(&mut self, id: Id) {
        self.draw_method_id = id;
    }

    
    pub fn set_perspective_id(&mut self, id: Id) {
        self.perspective_id = id;
    }

    
    pub fn set_view_id(&mut self, id: Id) {
        self.view_id = id;
    }

    
    pub fn set_model_id(&mut self, id: Id) {
        self.model_id = id;
    }

    
    pub fn get_vertex_id(&self) -> Id {
        self.vertex_id
    }

    
    pub fn get_index_id(&self) -> Id {
        self.index_id
    }

    
    pub fn get_draw_method_id(&self) -> Id {
        self.draw_method_id
    }

    
    pub fn get_perspective_id(&self) -> Id {
        self.perspective_id
    }

    
    pub fn get_view_id(&self) -> Id {
        self.view_id
    }

    
    pub fn get_model_id(&self) -> Id {
        self.model_id
    }
}

#[derive(Debug)]
pub enum RenderableVertexColorErr {
    // Poison(&'static str),
    RendererVertexColor(&'static str, RendererVertexColorErr),
}

impl fmt::Display for RenderableVertexColorErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // RenderableVertexColorErr::Poison(_) => write!(f, "Thread was Poisoned During R/W"),
            RenderableVertexColorErr::RendererVertexColor(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for RenderableVertexColorErr {
    fn description(&self) -> &str {
        match *self {
            // RenderableVertexColorErr::Poison(_) => "Thread was Poisoned",
            RenderableVertexColorErr::RendererVertexColor(_, ref err) => err.description(),
            // RenderableTex2Err::Window(_, ref err) => err.description(),
            // RenderableTex2Err::RendererTex2(_, ref err) => err.description(),
        }
    }
}
