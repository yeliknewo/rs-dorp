use logic::{Entity, World, Id};
use err::DorpErr;

#[derive(Debug)]
pub struct Named {
    name: &'static str,
}

impl Named {
    pub fn new<T: Entity<T>>(name: &'static str, id: Id, world: &mut World<T>) -> Result<Named, DorpErr> {
        match world.register_name(id, name) {
            Ok(_) => {
                Ok(
                    Named {
                        name: name,
                    }
                )
            },
            Err(err) => Err(DorpErr::DorpString("World Register Name: ".to_string() + name, Box::new(err))),
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}
