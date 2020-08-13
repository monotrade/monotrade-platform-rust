//use std::io;
use monotrade::platform::Platform;

//#[actix_rt::main] // <- starts the system and block until future resolves
fn main() {
    let platform = Platform::new(String::from("demo"));
    platform.initialize();
    platform.start();
}

