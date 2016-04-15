use std::collections::{HashMap};
use std::hash::{Hash};

use logic::{Id};
use err::DorpErr;

#[derive(Debug)]
pub struct Map3d<T: Hash + Eq + Copy> {
    tiles: HashMap<T, HashMap<T, HashMap<T, Id>>>,
    dirty_tiles: bool,
    ticks: i32,
}

impl<T: Hash + Eq + Copy> Map3d<T> {
    pub fn new() -> Map3d<T> {
        Map3d {
            tiles: HashMap::new(),
            dirty_tiles: false,
            ticks: 0,
        }
    }

    pub fn tick_mut(&mut self) {
        if self.dirty_tiles {
            self.ticks += 1;
            if self.ticks > 1 {
                self.dirty_tiles = false;
                self.ticks = 0;
            }
        }
    }

    pub fn insert(&mut self, x: T, y: T, z: T, id: Id) -> Result<(), DorpErr> {
        if !self.tiles.contains_key(&z) {
            self.tiles.insert(z, HashMap::new());
        }
        match self.tiles.get_mut(&z) {
            Some(mut plane) => {
                if !plane.contains_key(&y) {
                    plane.insert(y, HashMap::new());
                }
                match plane.get_mut(&y) {
                    Some(mut line) => {
                        line.insert(x, id);
                    },
                    None => return Err(DorpErr::Base("Plane Get Mut Y was none")),
                }
            },
            None => return Err(DorpErr::Base("Self Tiles Get Mut Z was none")),
        }
        Ok(())
    }

    pub fn get(&self, x: T, y: T, z: T) -> Option<Id> {
        match self.tiles.get(&z) {
            Some(plane) => match plane.get(&y) {
                Some(row) => match row.get(&x) {
                    Some(id) => Some(*id),
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }

    pub fn get_all(&self) -> &HashMap<T, HashMap<T, HashMap<T, Id>>> {
        &self.tiles
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty_tiles
    }
}
