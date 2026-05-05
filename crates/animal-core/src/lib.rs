use std::sync::Arc;

use libloading::os::unix::{Library, Symbol};

pub trait Animal {
    fn say(&self) -> &'static str;
}

pub trait AnimalFactory {
    fn factory() -> Box<dyn Animal + 'static>;
}

impl Animal for Box<dyn Animal> {
    fn say(&self) -> &'static str {
        self.as_ref().say()
    }
}

impl<E> Animal for Box<E>
where
    E: Animal,
{
    fn say(&self) -> &'static str {
        E::say(&*self)
    }
}

pub struct DynAnimalFactory<E> {
    _library: Arc<Library>,
    factory: Symbol<fn() -> E>,
}

impl<E> DynAnimalFactory<E> {
    pub fn build(&self) -> DynAnimal<E> {
        DynAnimal {
            _library: self._library.clone(),
            animal: (self.factory)(),
        }
    }
}

pub struct DynAnimal<E> {
    _library: Arc<Library>,
    animal: E,
}

impl<E> Animal for DynAnimal<E>
where
    E: Animal,
{
    fn say(&self) -> &'static str {
        (self.animal).say()
    }
}

impl dyn Animal {
    pub fn load_library(filename: &str) -> DynAnimalFactory<Box<dyn Animal + 'static>> {
        let library = unsafe { Library::new(filename).unwrap() };
        let factory: Symbol<fn() -> Box<dyn Animal>> = unsafe { library.get("factory").unwrap() };

        DynAnimalFactory {
            _library: Arc::new(library),
            factory: factory,
        }
    }
}

#[macro_export]
macro_rules! expose {
    ($animal:ty) => {
        const _: () = {
            fn assert_impl<T: $crate::Animal + $crate::AnimalFactory>(_: T) {}
            fn assert(animal_impl: $animal) {
                assert_impl(animal_impl);
            }
        };

        #[unsafe(no_mangle)]
        pub extern "Rust" fn factory() -> Box<dyn Animal + 'static> {
            <$animal as $crate::AnimalFactory>::factory()
        }

        #[unsafe(no_mangle)]
        pub extern "Rust" fn say(animal: &$animal) -> &'static str {
            <$animal as $crate::Animal>::say(animal)
        }
    };
}
