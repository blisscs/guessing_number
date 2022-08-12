use std::io;

fn main() {
    println!("Guess the number!");
    
    println!("Please input your guess.");

    let mut guess = String::new();

    // we can use sto::io::stdin if we do not import
    // std::io by `use std::io` on the first line
    // or we could write
    //io::stdin().read_line(&mut guess).expect("Failed to read line");
    // on a single line
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    // or you can write `println!("You guessed: {}", guess)`
}
