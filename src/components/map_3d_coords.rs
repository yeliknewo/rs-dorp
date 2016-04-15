use std::hash::Hash;

use logic::{Id};
use components::{Map3d};
use err::DorpErr;

#[derive(Debug)]
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

    pub fn register(&self, id: Id, map_3d: &mut Map3d<T>) -> Result<(), DorpErr> {
        match map_3d.insert(self.x, self.y, self.z, id) {
            Ok(()) => Ok(()),
            Err(err) => return Err(DorpErr::Dorp("Map 3d insert, x, y, z, id", Box::new(err))),
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
