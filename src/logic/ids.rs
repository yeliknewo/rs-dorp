use std::collections::{HashMap};
use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Id {
    id: IdSize,
}

impl Id {
    
    pub fn new(manager: &mut IdManager, id_type: IdType) -> Id {
        Id {
            id: manager.get_id(id_type),
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}", self.id)
    }
}

pub trait IdTypeTrait {

}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum IdType {
    Custom(Box<IdType>),
    World,
    Entity,
    Component,
    Vertex,
    Index,
    Texture,
    DrawMethod,
    Matrix,
    Color,
}

pub struct IdManager {
    map: HashMap<IdType, IdSize>,
}

impl IdManager {
    
    pub fn new() -> IdManager {
        IdManager {
            map: HashMap::new(),
        }
    }

    
    fn get_id(&mut self, id_type: IdType) -> IdSize {
        let id = match self.map.get(&id_type) {
            Some(id) => *id,
            None => 0,
        };
        self.map.insert(id_type, id + 1);
        id
    }
}

pub type IdSize = u64;
