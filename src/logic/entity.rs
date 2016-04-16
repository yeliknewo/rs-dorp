use std::sync::{Arc};

use logic::{World, Id, IdManager, TickCount};
use graphics::{Window, SyncData, Renderers};
use components::{Renderable, Named, Transform};
use err::DorpErr;

pub trait Entity<T: Entity<T>> : Send + Sync {
    fn tick(&self, TickCount, f64, Arc<World<T>>) -> Result<(), DorpErr>;
    fn tick_mut(&mut self, TickCount, &mut IdManager, &mut World<T>, &mut SyncData) -> Result<(), DorpErr>;
    fn render(&mut self, &mut Window, &mut SyncData, &mut Renderers) -> Result<(), DorpErr>;
    fn get_renderable(&self) -> Option<&Renderable>;
    fn get_named(&self) -> Option<&Named>;
    fn get_transform(&self) -> Option<&Transform>;
    fn get_id(&self) -> Id;
}
