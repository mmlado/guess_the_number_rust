use std::io;
use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1..100);
    loop {
        let input = get_input();

        if input.trim() == "exit" {
            println!("Exiting...");
            break;
        }
        
        let guess: i32;
        match input.trim().parse::<i32>() {
            Ok(n) => guess = n,
            Err(e) => {
                println!("Must be number or exit: {}", e);
                continue;
            },
        }

        if check(number, guess) {
            break;
        };
    }
}

fn get_input() -> String {
    println!("Guess the number (exit): ");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input
}

fn check(number: i32, guess: i32) -> bool {
    if guess == number {
        println!("You guessed right");
        return true;
    }
    
    if guess > number {
        println!("Guess lower!")
    }
    else {
        println!("Guess higher!")
    }

    false
}