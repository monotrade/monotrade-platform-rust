#[no_mangle]
pub extern fn fibonacci(n: i64) -> i64 {
  return match n {
    1 | 2 => 1,
    n => fibonacci(n - 1) + fibonacci(n - 2)
  }
}