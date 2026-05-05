use animal_core::Animal;

fn main() {
    let dog = {
        let factory = <dyn Animal>::load_library("./target/release/libdog.dylib");
        factory.build()
    };

    let cat = {
        let factory = <dyn Animal>::load_library("./target/release/libcat.dylib");
        factory.build()
    };

    for animal in [dog, cat] {
        println!("{}", animal.say());
    }
}
