use std::fmt;
use std::error::Error;

use logic::{EntityData, World, WorldErr, Id};

pub struct Named {
    name: &'static str,
}

impl Named {
    
    pub fn new<T: EntityData<T>>(name: &'static str, id: Id, world: &mut World<T>) -> Result<Named, NamedErr> {
        match world.register_name(id, name) {
            Ok(_) => {
                Ok(
                    Named {
                        name: name,
                    }
                )
            },
            Err(err) => Err(NamedErr::World("World Register Name", err)),
        }
    }

    
    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

#[derive(Debug)]
pub enum NamedErr {
    World(&'static str, WorldErr),
}

impl fmt::Display for NamedErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NamedErr::World(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for NamedErr {
    fn description(&self) -> &str {
        match *self {
            NamedErr::World(_, ref err) => err.description(),
        }
    }
}
