use std::collections::{HashMap};
use std::hash::{Hash};

use logic::{Id};

pub struct Map2d<T: Hash + Eq + Copy> {
    tiles: HashMap<T, HashMap<T, Id>>,
    dirty_tiles: bool,
    ticks: i32,
}

impl<T: Hash + Eq + Copy> Map2d<T> {
    
    pub fn new() -> Map2d<T> {
        Map2d {
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

    
    pub fn insert(&mut self, x: T, y: T, id: Id) {
        match self.tiles.remove(&y) {
            Some(mut row) => {
                row.insert(x, id);
                self.dirty_tiles = true;
                self.tiles.insert(y, row);
            },
            None => {
                self.tiles.insert(y, HashMap::new());
                self.insert(x, y, id);
            }
        }
    }

    
    pub fn get(&self, x: T, y: T) -> Option<Id> {
        match self.tiles.get(&y) {
            Some(row) => match row.get(&x) {
                Some(id) => Some(id.clone()),
                None => None,
            },
            None => None,
        }
    }

    
    pub fn get_all(&self) -> &HashMap<T, HashMap<T, Id>> {
        &self.tiles
    }

    
    pub fn is_dirty(&self) -> bool {
        self.dirty_tiles
    }
}
