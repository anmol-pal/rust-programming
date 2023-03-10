use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);
   
    // All variables are immutable by default, can not be changed
    // Binding return value of String::new() to mutable variable guess
    // String is a type in std lib, new is a function associated with String type indicated by ::
    let mut guess= String::new();
    println!("Please input your guess");

    //Reading intput from stdin and input is stored into guess and returns Result
    // We are passing guess as a ref -> & 
    // Expect is to handle error, io::Result type has an expect() method that exits program  with provided message if Resul is an error
    io::stdin().read_line(&mut guess).expect("Failed to read line!");
    println!("You guessed {}",guess);

    let x: i32 = guess.trim().parse().expect("Input not an integer");


    if (secret_number==x){
        println!("good guess");
    }else if(secret_number<x){
        println!("too high");
    }else {
        println!("too low");
    }

// Match will pattern matching where we compare >, <, = to our secret_number  
// cmp will return an ordering tyoe , that is an enum with 3 variants, each variant is a type too
//  Ordering::Greater [Pattern] => println! [Code to run if pattern matches]
// Statement ends with ; here we are seperating with ,   because they are expressions
match x.cmp(&secret_number){
    Ordering::Greater => println!("guess too high"),
    Ordering::Less => println!("guess too low"),
    Ordering::Equal => println!("You got it right"),
}
}
