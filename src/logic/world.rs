use std::collections::{HashMap};

use input::{Keyboard, Mouse, Display, KeyCode, MouseButton, Button};
use logic::{Id, Entity};
use math::{Vec2};
use err::DorpErr;

#[derive(Debug)]
pub struct World<T: Entity<T>> {
    keyboard: Keyboard,
    mouse: Mouse,
    display: Display,
    entities: HashMap<Id, T>,
    names: HashMap<&'static str, Id>,
    to_remove: Vec<Id>,
}

impl<T: Entity<T>> World<T> {
    pub fn new(keyboard: Keyboard, mouse: Mouse, display: Display) -> World<T> {
        World {
            keyboard: keyboard,
            mouse: mouse,
            display: display,
            entities: HashMap::new(),
            names: HashMap::new(),
            to_remove: vec!(),
        }
    }

    pub fn set_key(&mut self, key_code: KeyCode, key: Button) {
        self.keyboard.set_key_state(key_code, key);
    }

    pub fn set_mouse_button(&mut self, mouse_button: MouseButton, button: Button) {
        self.mouse.set_mouse_button(mouse_button, button);
    }

    pub fn set_mouse_position(&mut self, pos: Vec2)  {
        self.mouse.set_mouse_position(pos);
    }

    pub fn set_resolution(&mut self, resolution:  Vec2) {
        self.display.set_resolution(resolution);
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

    pub fn get_entities(&self) -> &HashMap<Id, T> {
        &self.entities
    }

    pub fn get_mut_entities(&mut self) -> &mut HashMap<Id, T> {
        &mut self.entities
    }

    pub fn add_entity(&mut self, entity: T) {
        self.entities.insert(entity.get_id(), entity);
    }

    pub fn tick_mut(&mut self) {
        let len = self.to_remove.len();
        for id in self.to_remove.drain(0..len) {
            self.entities.remove(&id);
        }
    }

    pub fn queue_remove_entity(&mut self, id: Id) {
        self.to_remove.push(id);
    }

    pub fn get_entity_by_id(&self, id: Id) -> Option<&T> {
        self.entities.get(&id)
    }

    pub fn get_mut_entity_by_id(&mut self, id: Id) -> Option<&mut T> {
        self.entities.get_mut(&id)
    }

    pub fn take_entity_by_id(&mut self, id: Id) -> Option<T> {
        self.entities.remove(&id)
    }

    pub fn get_entity_by_name(&self, name: &'static str) -> Option<&T> {
        if let Some(id) = self.names.get(name) {
            return self.get_entity_by_id(*id);
        }
        return None;
    }

    pub fn get_mut_entity_by_name(&mut self, name: &'static str) -> Option<&mut T> {
        let id = match self.names.get(name) {
            Some(id) => id.clone(),
            None => return None,
        };
        self.get_mut_entity_by_id(id)
    }

    pub fn register_name(&mut self, id: Id, name: &'static str) -> Result<(), DorpErr> {
        if !self.names.contains_key(name) {
            self.names.insert(name, id);
        }
        return Err(DorpErr::BaseString("Names already contains name: ".to_string() + name));
    }

    pub fn deregister_name(&mut self, name: &'static str) -> Result<(), DorpErr> {
        if self.names.remove(name).is_none() {
            return Err(DorpErr::BaseString("Can't deregister name '".to_string() + name + "' because name is not registered"));
        }
        return Ok(());
    }
}
