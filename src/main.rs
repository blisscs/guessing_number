use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    loop {

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

        let guess: u32 = match guess.trim().parse() { 
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");
        // or you can write `println!("You guessed: {}", guess)`

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
