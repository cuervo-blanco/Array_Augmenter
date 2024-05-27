
#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
enum Leader {
    Astronaut(Rc<RefCell<Astronaut>>),
    Hat(Rc<RefCell<Hat>>),
    None,
}
impl Leader {
    fn clone_astro(&self) -> Leader {
        match self {
            Leader::Astronaut(astronaut) => {
                Leader::Astronaut(astronaut.clone())
            }
            Leader::Hat(hat) => Leader::Hat(hat.clone()),
            Leader::None => Leader::None,
        }
    }
}
