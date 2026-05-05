use animal_core::{Animal, AnimalFactory, expose};

pub struct Cat;

impl Animal for Cat {
    fn say(&self) -> &'static str {
        "Meow"
    }
}

impl AnimalFactory for Cat {
    fn factory() -> Box<dyn Animal + 'static> {
        Box::new(Cat)
    }
}

expose!(Cat);
