use std::hash::Hash;

use logic::{Id};
use components::{Map2d};

pub struct Map2dCoords<T: Hash + Eq + Copy> {
    x: T,
    y: T,
}

impl<T: Hash + Eq + Copy> Map2dCoords<T> {

    pub fn new(x: T, y: T) -> Map2dCoords<T> {
        Map2dCoords {
            x: x,
            y: y,
        }
    }


    pub fn register(&self, id: Id, map_2d: &mut Map2d<T>) {
        map_2d.insert(self.x, self.y, id);
    }


    pub fn get_x(&self) -> T {
        self.x
    }


    pub fn get_y(&self) -> T {
        self.y
    }
}
