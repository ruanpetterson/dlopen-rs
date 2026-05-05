use animal_core::Animal;

fn main() {
    let dog = {
        let factory = <dyn Animal>::load_library("./target/release/libdog.dylib");
        factory.build()
    };

    println!("{}", dog.say());
}
