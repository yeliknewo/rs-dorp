use math::{Vec3, Mat4};
use components::{Renderable};
use err::DorpErr;

#[derive(Debug)]
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

    pub fn render(&mut self, renderable: &mut Renderable) -> Result<(), DorpErr> {
        if self.dirty_render {
            match renderable.set_model(Mat4::scalation_from_vec3(self.scalation) * Mat4::rotation_from_vec3(self.rotation) * Mat4::translation_from_vec3(self.position)) {
                Ok(()) => (),
                Err(err) => return Err(DorpErr::Dorp("Renderable Set Model", Box::new(err))),
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
