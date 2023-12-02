use std::cmp::Ordering;
use std::io;

use rand::Rng;


fn main() {
    println!("----------Guess the number!----------");
    println!("Guess a number between 1 and 100");
    let number = rand::thread_rng().gen_range(1..=100);
    //println!("Answer is {number}");
    loop {
        println!("Input your guess: > ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue
        };
        if guess < 1 || guess > 100 {
            println!("The secret number is betwee 1 and 100.");
            continue;
        }
        println!("You guessed {guess}");
        match guess.cmp(&number) {
            Ordering::Less => println!("Higher"),
            Ordering::Greater => println!("Lower"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


pub struct Guess {
    value: i32
}


impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be on [1, 100]. Got {value}.");
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}