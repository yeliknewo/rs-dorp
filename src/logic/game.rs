use std::sync::{Arc};
use scoped_threadpool::{Pool};
use time::{precise_time_s};
use glium::glutin::Event as WindowEvent;

use input::{Keyboard, Mouse, Display, KeyCode, ButtonState, MouseButton, Button};
use logic::{TickCount, World, Entity, IdManager};
use math::{Vec2};
use graphics::{Window, SyncData, Renderers};
use err::{DorpErr};

pub struct Game<T: Entity<T>> {
    world: Arc<World<T>>,
    sync_data: Arc<SyncData>,
    thread_pool: Pool,
    tick_count: TickCount,
}

impl<T: Entity<T>> Game<T> {
    pub fn new(thread_count: u32, resolution: Vec2) -> Game<T> {
        let keyboard = Arc::new(Keyboard::new());
        let mouse = Arc::new(Mouse::new());
        let display = Arc::new(Display::new(resolution));
        Game {
            world: Arc::new(World::new(keyboard.clone(), mouse.clone(), display.clone())),
            sync_data: Arc::new(SyncData::new()),
            thread_pool: Pool::new(thread_count),
            tick_count: 0,
        }
    }

    pub fn get_world(&self) -> Arc<World<T>> {
        self.world.clone()
    }

    pub fn get_mut_world(&mut self) -> Result<&mut World<T>, DorpErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => Ok(world),
            None => Err(DorpErr::Base("Arc Get Mut Self World")),
        }
    }

    fn pause(&mut self) {
        println!("Paused");
    }

    fn resume(&mut self) {
        println!("Resumed");
    }

    fn update_keyboard(&mut self, tick_number: u64, key_code: KeyCode, element_state: ButtonState) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_key(key_code, Button::new(tick_number, element_state)) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(DorpErr::Dorp("World Set Key", Box::new(err))),
                }
            },
            None => Err(DorpErr::Base("Arc Get Mut Self World was none")),
        }
    }

    fn update_mouse_button(&mut self, tick_number: u64, mouse_button: MouseButton, element_state: ButtonState) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_mouse_button(mouse_button, Button::new(tick_number, element_state)) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(DorpErr::Dorp("World Set Mouse Button", Box::new(err))),
                }
            }
            None => Err(DorpErr::Base("Arc Get Mut Self World was none")),
        }
    }

    fn update_mouse_pos(&mut self, mouse_pos: (i32, i32)) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_mouse_position(Vec2::from([mouse_pos.0 as f32, mouse_pos.1 as f32])) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(DorpErr::Dorp("World Set Mouse Position", Box::new(err))),
                }
            },
            None => Err(DorpErr::Base("Arc Get Mut Self World was none")),
        }
    }

    fn update_resolution(&mut self, resolution: (u32, u32)) -> Result<(), DorpErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_resolution(Vec2::from([resolution.0 as f32, resolution.1 as f32])) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(DorpErr::Dorp("World Set Resolution", Box::new(err))),
                }
            },
            None => Err(DorpErr::Base("Arc Get Mut Self World was none")),
        }
    }

    pub fn run(&mut self, window: &mut Window, manager: &mut IdManager) -> Result<(), DorpErr> {
        let mut renderers = match Renderers::new(window) {
            Ok(renderers) => renderers,
            Err(err) => return Err(DorpErr::Dorp("Renderers New", Box::new(err))),
        };

        let tps: f64 = 60.0;
        let tps_s: f64 = 1.0 / tps;

        let mut last_time: f64 = precise_time_s();
        let mut delta_time: f64 = 0.0;

        let mut i: f64 = last_time;

        let mut frames: u64 = 0;
        let mut ticks: u64 = 0;

        let mut tick_number: u64 = 0;

        loop {
            let now = precise_time_s();
            delta_time += now - last_time;
            last_time = now;
            while delta_time > 0.0 {
                for event in window.poll_events(){
                    match event {
                        WindowEvent::Resized(width, height) => match self.update_resolution((width, height)) {
                            Ok(()) => (),
                            Err(err) => return Err(DorpErr::Dorp("Self Update Resolution", Box::new(err))),
                        },
                        // WindowEvent::Moved(x, y) => {
                        //
                        // },
                        WindowEvent::Closed => return Ok(()),
                        // WindowEvent::DroppedFile(path_buffer) => {
                        //
                        // },
                        // WindowEvent::ReceivedCharacter(character) => {
                        //
                        // },
                        WindowEvent::Focused(focused) => {
                            if focused {
                                self.resume();
                            } else {
                                self.pause();
                            }
                        },
                        WindowEvent::KeyboardInput(element_state, _, virtual_key_code) => match virtual_key_code {
                            Some(virtual_key_code) => match self.update_keyboard(tick_number, virtual_key_code, element_state) {
                                Ok(()) => (),
                                Err(err) => return Err(DorpErr::Dorp("Self Update Keyboard", Box::new(err))),
                            },
                            None => (),
                        },
                        WindowEvent::MouseMoved(pos) => match self.update_mouse_pos(pos) {
                            Ok(()) => (),
                            Err(err) => return Err(DorpErr::Dorp("Self Update Mouse Pos", Box::new(err))),
                        },
                        // WindowEvent::MouseWheel(mouse_scroll_data) => {
                        //
                        // },
                        WindowEvent::MouseInput(element_state, mouse_button) => match self.update_mouse_button(tick_number, mouse_button, element_state) {
                            Ok(()) => (),
                            Err(err) => return Err(DorpErr::Dorp("Self Update Mouse Button", Box::new(err))),
                        },
                        // WindowEvent::Awakened => {
                        //
                        // },
                        // WindowEvent::Refresh => {
                        //
                        // },
                        // WindowEvent::Suspended(suspended) => {
                        //
                        // },
                        // WindowEvent::Touch(touch) => {
                        //
                        // },
                        _ => (),
                    }
                }
                match self.tick(tps_s, manager) {
                    Ok(()) => (),
                    Err(err) => return Err(DorpErr::Dorp("Self Tick", Box::new(err))),
                };
                delta_time -= tps_s;
                ticks += 1;
                tick_number += 1;
            }
            renderers = match self.render(window, renderers) {
                Ok(renderers) => renderers,
                Err(err) => return Err(DorpErr::Dorp("Self Render", Box::new(err))),
            };
            frames += 1;
            if now > i + 1.0 {
                i += 1.0;
                println!("Frames: {} Ticks: {}", frames.to_string(), ticks.to_string());
                frames = 0;
                ticks = 0;
            }
        }
    }

    fn render(&mut self, window: &mut Window, renderers: Renderers) -> Result<Renderers, DorpErr> {
        let mut renderers = renderers;
        let mut world = match Arc::get_mut(&mut self.world) {
            Some(world) => world,
            None => return Err(DorpErr::Base("Arc Get Mut Self World was none")),
        };
        for (_, entity) in match world.get_mut_entities() {
            Ok(entity) => entity,
            Err(err) => return Err(DorpErr::Dorp("World Get Mut Entity", Box::new(err))),
        }.iter_mut() {
            match match Arc::get_mut(entity) {
                    Some(entity) => entity,
                    None => return Err(DorpErr::Base("Arc Get Mut Entity was none")),
                }.render(window, match Arc::get_mut(&mut self.sync_data) {
                    Some(sync_data) => sync_data,
                    None => return Err(DorpErr::Base("Arc Get Mut Self Matrix Data was none")),
                }, &mut renderers) {
                Ok(()) => (),
                Err(err) => return Err(DorpErr::Dorp("Entity Render", Box::new(err))),
            }
        }
        match world.tick_mut() {
            Ok(()) => (),
            Err(err) => return Err(DorpErr::Dorp("World Tick Mut", Box::new(err))),
        }
        let mut frame = window.frame(renderers);
        for entry in match world.get_mut_entities() {
            Ok(entity) => entity,
            Err(err) => {
                match frame.end() {
                    Ok(_) => (),
                    Err(err) => return Err(DorpErr::Dorp("Frame End", Box::new(err))),
                };
                return Err(DorpErr::Dorp("World Get Mut Entity Data", Box::new(err)))
            },
        }.iter() {
            match frame.draw_entity(entry.1.as_ref(), self.sync_data.as_ref()) {
                Ok(()) => (),
                Err(err) => {
                    match frame.end() {
                        Ok(_) => (),
                        Err(err) => return Err(DorpErr::Dorp("Frame End", Box::new(err))),
                    }
                    return Err(DorpErr::Dorp("Frame Draw Entity", Box::new(err)))
                },
            };
        }
        match frame.end() {
            Ok(renderers) => Ok(renderers),
            Err(err) => Err(DorpErr::Dorp("Frame End", Box::new(err))),
        }
    }

    fn tick(&mut self, delta_time: f64, manager: &mut IdManager) -> Result<(), DorpErr> {
        {
            let world = self.world.clone();
            let delta_time = Arc::new(delta_time);
            let tick_count = Arc::new(self.tick_count);
            self.thread_pool.scoped(|scope| {
                for entry in world.get_entities().iter() {
                    let entity = entry.1.clone();
                    let world = world.clone();
                    let delta_time = delta_time.clone();
                    let tick_count = tick_count.clone();
                    scope.execute(move || {
                        match entity.tick(tick_count, delta_time, world) {
                            Ok(()) => (),
                            Err(err) => Err(("Entity Tick", err)).unwrap(),
                        }
                    });
                }
            });
        }
        match Arc::get_mut(&mut self.world)  {
            Some(world) => {
                let mut keys = vec!();
                match world.get_mut_entities() {
                    Ok(entity) => {
                        for key in entity.keys() {
                            keys.push(key.clone());
                        }
                    },
                    Err(err) => return Err(DorpErr::Dorp("World Get Mut Entity", Box::new(err))),
                }
                for key in keys {
                    let mut entity: Arc<T> = {
                        match world.get_mut_entities() {
                            Ok(entities) => {
                                match entities.remove(&key) {
                                    Some(entity) => entity,
                                    None => return Err(DorpErr::Base("Entities Remove was none")),
                                }
                            },
                            Err(err) => return Err(DorpErr::Dorp("World Get Mut Entities", Box::new(err))),
                        }
                    };
                    match match Arc::get_mut(&mut entity) {
                        Some(entity) => entity,
                        None => return Err(DorpErr::Base("Arc Get Mut Entity was none")),
                    }.tick_mut(self.tick_count, manager, world, match Arc::get_mut(&mut self.sync_data) {
                        Some(matrix_data) => matrix_data,
                        None => return Err(DorpErr::Base("Arc Get Mut Self Matrix Data was none")),
                    }) {
                        Ok(()) => (),
                        Err(err) => return Err(DorpErr::Dorp("Entity Tick Mut", Box::new(err))),
                    }
                    match world.get_mut_entities() {
                        Ok(entities) => {
                            entities.insert(key, entity);
                        }
                        Err(err) => return Err(DorpErr::Dorp("World Get Mut Entity", Box::new(err))),
                    }
                }
            },
            None => return Err(DorpErr::Base("Arc Get Mut Self World was none")),
        }
        self.tick_count += 1;
        Ok(())
    }
}
