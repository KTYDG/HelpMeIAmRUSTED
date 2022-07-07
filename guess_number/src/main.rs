use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Make a guess:");

    let number_to_guess = thread_rng().gen_range(1..=100);
    //println!("Number to guess: {number_to_guess}");

    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("WTF happened?!");
        // let guess: u32 = guess.trim().parse().expect("You didn't entered number");
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Not a number or integer!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
