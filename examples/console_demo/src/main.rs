use std::io;
use monotrade::platform::Platform;

#[actix_rt::main] // <- starts the system and block until future resolves
async fn main() {
    let platform = Platform::new(String::from("demo"));
    platform.initialize();
}

