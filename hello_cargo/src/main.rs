use std::io;
fn main() {
    println!("Hello, world!");
    println!("Guess the number");
    // All variables are immutable by default, can not be changed
    // Binding return value of String::new() to mutable variable guess
    // String is a type in std lib, new is a function associated with String type indicated by ::
    let mut guess= String::new();
    println!("Please input your guess");

    //Reading intput from stdin and input is stored into guess 
    //We are passing guess as a ref -> & 
    io::stdin().read_line(&mut guess).expect("Failed to erad line!");
    println!("You guessed {}",guess);

}
