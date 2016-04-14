use std::fmt;
use std::error::Error;

use math::{Vec3, Mat4};
use components::{Renderable, RenderableErr};

pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scalation: Vec3,
    dirty_render: bool,
}

impl Transform {
    
    pub fn new() -> Transform {
        Transform {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scalation: Vec3::one(),
            dirty_render: true,
        }
    }

    
    pub fn render(&mut self, renderable: &mut Renderable) -> Result<(), TransformErr> {
        if self.dirty_render {
            match renderable.set_model(Mat4::scalation_from_vec3(self.scalation) * Mat4::rotation_from_vec3(self.rotation) * Mat4::translation_from_vec3(self.position)) {
                Ok(()) => (),
                Err(err) => return Err(TransformErr::Renderable("Renderable Set Model", err)),
            }
            self.dirty_render = false;
        }
        Ok(())
    }

    
    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
        self.dirty_render = true;
    }

    
    pub fn set_rotation(&mut self, rot: Vec3) {
        self.rotation = rot;
        self.dirty_render = true;
    }

    
    pub fn set_scalation(&mut self, sca: Vec3) {
        self.scalation = sca;
        self.dirty_render = true;
    }

    
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    
    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }

    
    pub fn get_scalation(&self) -> Vec3 {
        self.scalation
    }
}

#[derive(Debug)]
pub enum TransformErr {
    Poison(&'static str),
    Transform(&'static str, Box<TransformErr>),
    Renderable(&'static str, RenderableErr),
}

impl fmt::Display for TransformErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransformErr::Poison(_) => write!(f, "Thread was Poisoned During R/W"),
            TransformErr::Transform(_, ref err) => err.fmt(f),
            TransformErr::Renderable(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for TransformErr {
    fn description(&self) -> &str {
        match *self {
            TransformErr::Poison(_) => "Thread was Poisoned",
            TransformErr::Transform(_, ref err) => err.description(),
            TransformErr::Renderable(_, ref err) => err.description(),
        }
    }
}
