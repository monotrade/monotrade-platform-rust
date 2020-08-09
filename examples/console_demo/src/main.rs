use std::io;


fn main() {
    let platform = Platform::new();
    platform.initialize();

    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("Your guessed: {}", guess);

    unsafe { say_hello(); }
}


#[link(name = "test", kind = "static")]
extern "C" {
    pub fn say_hello();
}
