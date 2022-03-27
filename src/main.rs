// Pulls modules, traits and enums into scope.
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 10);

    // Loops the code indefinitely.
    loop{
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converts the string into a unsigned integer.
        let guess : u32 = guess.trim().parse().
        expect("Please type a number!");

        println!("You guessed: {}", guess);

        // Compares the secret number against the guess.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("you win!"),
        }
    }
}
