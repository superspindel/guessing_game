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
        let guess: u32 = match get_user_input()
        {
            Ok(number) => number,
            Err(error_info) => 
            {
                println!("{}", error_info);
                continue;
            }
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

fn get_user_input() -> Result<u32, String> {
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess)
    {
        Ok(_) => {
            match guess.trim().parse()
            {
                Ok(num) => Ok(num),
                Err(error) => 
                {
                    let mut error_info = String::from(error.to_string());
                    error_info.insert_str(0, "in parsing u32, ");
                    Err(error_info)
                },
            }
        }
        Err(error) => Err(error.to_string()),
    }
}