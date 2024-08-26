use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Processing a guess.
fn main() {
    println!("Guess what the number is!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input a guess.");

        let mut guess = String::new(); // mutable variable currently bound to a new, empty instance of a String

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Comparing the user's guess to the secret random number generated.
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

