use animal_core::{Animal, AnimalFactory, expose};

pub struct Dog;

impl Animal for Dog {
    fn say(&self) -> &'static str {
        "Auau"
    }
}

impl AnimalFactory for Dog {
    fn factory() -> Box<dyn Animal + 'static> {
        Box::new(Dog)
    }
}

expose!(Dog);
