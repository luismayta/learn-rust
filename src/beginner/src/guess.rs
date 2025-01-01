use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret_number is {secret_number}");

    println!("please input the guess the number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("please type a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too Small");
        }
        Ordering::Greater => {
            println!("Too Greater");
        }
        Ordering::Equal => {
            println!("You Win");
        }
    }

    println!("You Guessed {}", guess);
}