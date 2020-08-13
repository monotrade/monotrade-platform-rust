use monotrade::platform::Platform;

#[no_mangle]
pub extern fn fibonacci(n: i64) -> i64 {
  return match n {
    1 | 2 => 1,
    n => fibonacci(n - 1) + fibonacci(n - 2)
  }
}


//static platform:Platform = Platform{ name: "demo".to_string() };

#[no_mangle]
pub extern fn start() {
  let platform:Platform = Platform::new(String::from("demo"));
  platform.initialize();
  platform.start();
}