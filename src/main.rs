extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut number_of_tries = 0;
    loop {
        println!("Please input your guess");
        number_of_tries += 1;
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Your input was invalid, please try again!");
                continue;
                },
        };
        println!("You guessed: {}", guess);
        print!("Number of tries: ");    
        println!("{}", number_of_tries);
        match guess.cmp(&secret_number)
        {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win");
                break;
            }
        }
    
    }
}
