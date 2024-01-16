use std::io;
use rand::Rng;

fn main() {
    // Generate a random number between 1 and 100
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess a number!");
        // Create a new mutable string
        let mut guess = String::new();

        // Read the line from the standard input and store it in the mutable string
        println!("Please input your guess: ");
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        print!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
