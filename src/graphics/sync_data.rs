use std::collections::{HashMap};

use logic::{Id};
use math::{Mat4, Vec4};

pub struct SyncData {
    mat4s: HashMap<Id, Mat4>,
    mat4s_inverse: HashMap<Id, Mat4>,
    vec4s: HashMap<Id, Vec4>,
}

impl SyncData {
    
    pub fn new() -> SyncData {
        SyncData {
            mat4s: HashMap::new(),
            mat4s_inverse: HashMap::new(),
            vec4s: HashMap::new(),
        }
    }

    
    pub fn set_vec4(&mut self, id: Id, vec4: Vec4) {
        self.vec4s.insert(id, vec4);
    }

    
    pub fn set_matrix(&mut self, id: Id, mat4: Mat4, inverse: Mat4) {
        self.mat4s.insert(id, mat4);
        self.mat4s_inverse.insert(id, inverse);
    }

    
    pub fn get_vec4(&self, id: Id) -> Option<&Vec4> {
        self.vec4s.get(&id)
    }

    
    pub fn get_matrix(&self, id: Id) -> Option<&Mat4> {
        self.mat4s.get(&id)
    }

    
    pub fn get_inverse(&self, id: Id) -> Option<&Mat4> {
        self.mat4s_inverse.get(&id)
    }

}
