use std::sync::{Arc};
use std::collections::{HashMap};
use std::fmt;
use std::error::Error;

use input::{Keyboard, Mouse, Display, KeyCode, MouseButton, Button};
use logic::{Id, EntityData, OptErr};
use math::{Vec2};

pub struct World<T: EntityData<T>> {
    keyboard: Arc<Keyboard>,
    mouse: Arc<Mouse>,
    display: Arc<Display>,
    entity_data: Arc<HashMap<Id, Arc<T>>>,
    names: Arc<HashMap<&'static str, Id>>,
    to_remove: Vec<Id>,
}

impl<T: EntityData<T>> World<T> {

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


    pub fn set_key(&mut self, key_code: KeyCode, key: Button) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.keyboard){
            Some(keyboard) => {
                keyboard.set_key_state(key_code, key);
                Ok(())
            },
            None => Err(WorldErr::GetMut("Arc Get Mut Self Keyboard")),
        }
    }


    pub fn set_mouse_button(&mut self, mouse_button: MouseButton, button: Button) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.mouse) {
            Some(mouse) => {
                mouse.set_mouse_button(mouse_button, button);
                Ok(())
            },
            None => Err(WorldErr::GetMut("Arc Get Mut Self Mouse")),
        }
    }


    pub fn set_mouse_position(&mut self, pos: Vec2) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.mouse) {
            Some(mouse) => {
                mouse.set_mouse_position(pos);
                Ok(())
            },
            None => Err(WorldErr::GetMut("Arc Get Mut Self Mouse")),
        }
    }


    pub fn set_resolution(&mut self, resolution:  Vec2) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.display) {
            Some(display) => {
                display.set_resolution(resolution);
                Ok(())
            },
            None => Err(WorldErr::GetMut("Arc Get Mut Self Display")),
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


    pub fn get_entity_data(&self) -> Arc<HashMap<Id, Arc<T>>> {
        self.entity_data.clone()
    }


    pub fn get_mut_entity_data(&mut self) -> Result<&mut HashMap<Id, Arc<T>>, WorldErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => Ok(entity_data),
            None => Err(WorldErr::GetMut("Arc Get Mut Self EntityData")),
        }
    }


    pub fn add_entity(&mut self, entity: T) -> Result<(), WorldErr> {
        self.add_entity_arc(Arc::new(entity))
    }

    pub fn add_entity_arc(&mut self, entity: Arc<T>) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => {
                entity_data.insert(entity.get_id(), entity);
                Ok(())
            },
            None => Err(WorldErr::GetMut("Arc Get Mut Self EntityData")),
        }
    }

    pub fn tick_mut(&mut self) -> Result<(), WorldErr> {
        let len = self.to_remove.len();
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => {
                for id in self.to_remove.drain(0..len) {
                    entity_data.remove(&id);
                }
            },
            None => return Err(WorldErr::GetMut("Arc Get Mut")),
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


    pub fn get_mut_entity_by_id(&mut self, id: Id) -> OptErr<&mut T, WorldErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => match entity_data.get_mut(&id) {
                Some(entity) => match Arc::get_mut(entity) {
                    Some(entity) => return OptErr::Full(entity),
                    None => return OptErr::Error(WorldErr::GetMut("Arc Get mut Entity")),
                },
                None => return OptErr::Empty,
            },
            None => return OptErr::Error(WorldErr::GetMut("Arc Get Mut &mut Self Entity Data")),
        }
    }


    pub fn take_entity_by_id(&mut self, id: Id) -> OptErr<Arc<T>, WorldErr> {
        match Arc::get_mut(&mut self.entity_data) {
            Some(entity_data) => match entity_data.remove(&id) {
                Some(entity) => return OptErr::Full(entity),
                None => return OptErr::Empty,
            },
            None => return OptErr::Error(WorldErr::GetMut("Arc Get Mut Mut self entity data")),
        }
    }


    pub fn get_entity_by_name(&self, name: &'static str) -> Option<Arc<T>> {
        match self.names.get(name) {
            Some(id) => self.get_entity_by_id(*id),
            None => None,
        }
    }


    pub fn get_mut_entity_by_name(&mut self, name: &'static str) -> OptErr<&mut T, WorldErr> {
        let id = *(match self.names.get(name) {
            Some(id) => id,
            None => return OptErr::Empty,
        });
        self.get_mut_entity_by_id(id)
    }


    pub fn register_name(&mut self, id: Id, name: &'static str) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.names) {
            Some(names) => {
                if names.contains_key(name) {
                    Err(WorldErr::InvalidName("Names ContainsKey Name"))
                } else {
                    names.insert(name, id);
                    Ok(())
                }
            },
            None => Err(WorldErr::GetMut("Arc Get Mut Self Names")),
        }
    }


    pub fn deregister_name(&mut self, name: &'static str) -> Result<(), WorldErr> {
        match Arc::get_mut(&mut self.names) {
            Some(names) => {
                names.remove(name);
                Ok(())
            },
            None => Err(WorldErr::GetMut("Arc Get Mut Self Names"))
        }
    }
}

#[derive(Debug)]
pub enum WorldErr {
    GetMut(&'static str),
    InvalidName(&'static str),
}

impl fmt::Display for WorldErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WorldErr::GetMut(_) => write!(f, "Get Mut was None"),
            WorldErr::InvalidName(_) => write!(f, "Name was already taken"),
        }
    }
}

impl Error for WorldErr {
    fn description(&self) -> &str {
        match *self {
            WorldErr::GetMut(_) => "Get Mut was None",
            WorldErr::InvalidName(_) => "Invalid Name",
        }
    }
}
