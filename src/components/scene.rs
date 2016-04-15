use std::error::Error;

use logic::{IdManager, World, Entity, Id};
use graphics::{SyncData};

pub struct Scene<T: Entity<T>> {
    setup: Box<Fn(&mut IdManager, &mut World<T>, &mut SyncData) -> Result<(), Box<Error>> + Send + Sync>,
}

impl<T: Entity<T>> Scene<T> {
    pub fn new(setup: Box<Fn(&mut IdManager, &mut World<T>, &mut SyncData) -> Result<(), Box<Error>> + Send + Sync>) -> Scene<T> {
        Scene {
            setup: setup,
        }
    }

    pub fn tick_mut(&mut self, my_id: Id, manager: &mut IdManager, world: &mut World<T>, sync_data: &mut SyncData) -> Result<(), Box<Error>> {
        world.queue_remove_entity(my_id);
        (self.setup)(manager, world, sync_data)
    }
}
