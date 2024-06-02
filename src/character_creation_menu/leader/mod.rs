use std::cell::RefCell;
use std::rc::Rc;

use crate::character_creation_menu::astronaut;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Leader {
    Astronaut(Rc<RefCell<astronaut::Astronaut>>),
    Hat(Rc<RefCell<astronaut::Hat>>),
    None,
}
impl Leader {
    pub fn clone_astro(&self) -> Leader {
        match self {
            Leader::Astronaut(astronaut) => {
                Leader::Astronaut(astronaut.clone())
            }
            Leader::Hat(hat) => Leader::Hat(hat.clone()),
            Leader::None => Leader::None,
        }
    }
}
