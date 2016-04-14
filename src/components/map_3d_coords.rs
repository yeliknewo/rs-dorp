use std::hash::Hash;
use std::error::Error;
use std::fmt;

use logic::{Id};
use components::{Map3d, Map3dErr};

pub struct Map3dCoords<T: Hash + Eq + Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T: Hash + Eq + Copy> Map3dCoords<T> {

    pub fn new(x: T, y: T, z: T) -> Map3dCoords<T> {
        Map3dCoords {
            x: x,
            y: y,
            z: z,
        }
    }


    pub fn register(&self, id: Id, map_3d: &mut Map3d<T>) -> Result<(), Map3dCoordsErr> {
        match map_3d.insert(self.x, self.y, self.z, id) {
            Ok(()) => Ok(()),
            Err(err) => return Err(Map3dCoordsErr::Map3d("Map 3d insert, x, y, z, id", err)),
        }
    }


    pub fn get_x(&self) -> T {
        self.x
    }


    pub fn get_y(&self) -> T {
        self.y
    }


    pub fn get_z(&self) -> T {
        self.z
    }
}

#[derive(Debug)]
pub enum Map3dCoordsErr {
    // World(&'static str, WorldErr),
    Map3d(&'static str, Map3dErr),
}

impl fmt::Display for Map3dCoordsErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // NamedErr::World(_, ref err) => err.fmt(f),
            Map3dCoordsErr::Map3d(_, ref err) => err.fmt(f),
        }
    }
}

impl Error for Map3dCoordsErr {
    fn description(&self) -> &str {
        match *self {
            // NamedErr::World(_, ref err) => err.description(),
            Map3dCoordsErr::Map3d(_, ref err) => err.description(),
        }
    }
}
