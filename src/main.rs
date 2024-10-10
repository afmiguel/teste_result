use std::io;

fn main() {
    println!("Please enter a number:");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .unwrap();  // Can cause a panic if reading the line fails

    let number: i32 = user_input.trim().parse()
        .unwrap();  // Can cause a panic if the input cannot be converted to i32

    println!("You entered the number: {}", number);
}


