use logic::{IdManager, World, Entity, Id};
use graphics::{SyncData};
use err::DorpErr;

pub struct Scene<T: Entity<T>> {
    setup: Box<Fn(&mut IdManager, &mut World<T>, &mut SyncData) -> Result<(), DorpErr> + Send + Sync>,
}

impl<T: Entity<T>> Scene<T> {
    pub fn new(setup: Box<Fn(&mut IdManager, &mut World<T>, &mut SyncData) -> Result<(), DorpErr> + Send + Sync>) -> Scene<T> {
        Scene {
            setup: setup,
        }
    }

    pub fn tick_mut(&mut self, my_id: Id, manager: &mut IdManager, world: &mut World<T>, sync_data: &mut SyncData) -> Result<(), DorpErr> {
        world.queue_remove_entity(my_id);
        (self.setup)(manager, world, sync_data)
    }
}
