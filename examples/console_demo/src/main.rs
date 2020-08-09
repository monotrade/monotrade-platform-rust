use std::io;
use monotrade::platform::Platform;

fn main() {
    let platform = Platform::new(String::from("demo"));
    platform.initialize();
}

