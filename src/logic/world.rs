use std::sync::{Arc};
use std::collections::{HashMap};

use input::{Keyboard, Mouse, Display, KeyCode, MouseButton, Button};
use logic::{Id, Entity, OptErr};
use math::{Vec2};
use err::DorpErr;

#[derive(Debug)]
pub struct World<T: Entity<T>> {
    keyboard: Arc<Keyboard>,
    mouse: Arc<Mouse>,
    display: Arc<Display>,
    entity_data: Arc<HashMap<Id, Arc<T>>>,
    names: Arc<HashMap<&'static str, Id>>,
    to_remove: Vec<Id>,
}

impl<T: Entity<T>> World<T> {
    pub fn new(keyboard: Arc<Keyboard>, mouse: Arc<Mouse>, display: Arc<Display>) -> World<T> {
        World {
            keyboard: keyboard,
            mouse: mouse,
            display: display,
            entity_data: Arc::new(HashMap::new()),
            names: Arc::new(HashMap::new()),
            to_remove: vec!(),
        }
    }

    pub fn set_key(&mut self, key_code: KeyCode, key: Button) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.keyboard){
            Some(keyboard) => {
                keyboard.set_key_state(key_code, key);
                Ok(())
            },
            None => Err(DorpErr::Base("Arc Get Mut Self Keyboard is none")),
        }
    }

    pub fn set_mouse_button(&mut self, mouse_button: MouseButton, button: Button) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.mouse) {
            Some(mouse) => {
                mouse.set_mouse_button(mouse_button, button);
                Ok(())
            },
            None => Err(DorpErr::Base("Arc Get Mut Self Mouse is none")),
        }
    }

    pub fn set_mouse_position(&mut self, pos: Vec2) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.mouse) {
            Some(mouse) => {
                mouse.set_mouse_position(pos);
                Ok(())
            },
            None => Err(DorpErr::Base("Arc Get Mut Self Mouse is none")),
        }
    }

    pub fn set_resolution(&mut self, resolution:  Vec2) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.display) {
            Some(display) => {
                display.set_resolution(resolution);
                Ok(())
            },
            None => Err(DorpErr::Base("Arc Get Mut Self Display is none")),
        }
    }

    pub fn get_key(&self, key_code: KeyCode) -> Button {
        self.keyboard.get_key(key_code)
    }

    pub fn get_mouse_button(&self, mouse_button: MouseButton) -> Button {
        self.mouse.get_button(mouse_button)
    }

    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse.get_mouse_position()
    }

    pub fn get_resolution(&self) -> Vec2 {
        self.display.get_resolution()
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.display.get_aspect_ratio()
    }

    pub fn get_entities(&self) -> Arc<HashMap<Id, Arc<T>>> {
        self.entity_data.clone()
    }

    pub fn get_mut_entities(&mut self) -> Result<&mut HashMap<Id, Arc<T>>, DorpErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => Ok(entity_data),
            None => Err(DorpErr::Base("Arc Get Mut Self EntityData is none")),
        }
    }

    pub fn add_entity(&mut self, entity: T) -> Result<(), DorpErr> {
        self.add_entity_arc(Arc::new(entity))
    }

    pub fn add_entity_arc(&mut self, entity: Arc<T>) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => {
                entity_data.insert(entity.get_id(), entity);
                Ok(())
            },
            None => Err(DorpErr::Base("Arc Get Mut Self EntityData is none")),
        }
    }

    pub fn tick_mut(&mut self) -> Result<(), DorpErr> {
        let len = self.to_remove.len();
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => {
                for id in self.to_remove.drain(0..len) {
                    entity_data.remove(&id);
                }
            },
            None => return Err(DorpErr::Base("Arc Get Mut is none")),
        }
        Ok(())
    }

    pub fn queue_remove_entity(&mut self, id: Id) {
        self.to_remove.push(id);
    }

    pub fn get_entity_by_id(&self, id: Id) -> Option<Arc<T>> {
        match self.entity_data.get(&id) {
            Some(entity_data) => {
                Some(entity_data.clone())
            }
            None => None,
        }
    }

    pub fn get_mut_entity_by_id(&mut self, id: Id) -> OptErr<&mut T, DorpErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => match entity_data.get_mut(&id) {
                Some(entity) => match Arc::get_mut(entity) {
                    Some(entity) => return OptErr::Full(entity),
                    None => return OptErr::Error(DorpErr::Base("Arc Get mut Entity is none")),
                },
                None => return OptErr::Empty,
            },
            None => return OptErr::Error(DorpErr::Base("Arc Get Mut &mut Self Entity Data is none")),
        }
    }

    pub fn take_entity_by_id(&mut self, id: Id) -> OptErr<Arc<T>, DorpErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => match entity_data.remove(&id) {
                Some(entity) => return OptErr::Full(entity),
                None => return OptErr::Empty,
            },
            None => return OptErr::Error(DorpErr::Base("Arc Get Mut Mut self entity data is none")),
        }
    }

    pub fn get_entity_by_name(&self, name: &'static str) -> Option<Arc<T>> {
        match self.names.get(name) {
            Some(id) => self.get_entity_by_id(*id),
            None => None,
        }
    }

    pub fn get_mut_entity_by_name(&mut self, name: &'static str) -> OptErr<&mut T, DorpErr> {
        let id = *(match self.names.get(name) {
            Some(id) => id,
            None => return OptErr::Empty,
        });
        self.get_mut_entity_by_id(id)
    }

    pub fn register_name(&mut self, id: Id, name: &'static str) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.names) {
            Some(names) => {
                if names.contains_key(name) {
                    Err(DorpErr::Base("Names Contains Key Name"))
                } else {
                    names.insert(name, id);
                    Ok(())
                }
            },
            None => Err(DorpErr::Base("Arc Get Mut Self Names is none")),
        }
    }

    pub fn deregister_name(&mut self, name: &'static str) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.names) {
            Some(names) => {
                names.remove(name);
                Ok(())
            },
            None => Err(DorpErr::Base("Arc Get Mut Self Names is none"))
        }
    }
}
