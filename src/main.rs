use std::{io, cmp::Ordering};
use rand::Rng;


fn main() {
  //guessing_game();
  let sum = sum_multiples_of_three_and_five(1000);
  println!("{sum}");
  
}

fn guessing_game() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    loop {
        
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // rust allows shadowing of the previous variable, that is why 
        // we can do this 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("**********************");
        println!("You guessed: {guess}");
        println!("**********************");

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

/// Returns the sum of all the numbers up to n
/// where i are multiples of 5 or 3
fn sum_multiples_of_three_and_five(n :u32) -> u32
{
    let mut sum = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0
        {
            sum += i;
        }
    }
    // we do not need a return statement we just
    // write the value we want to return without the ;
    sum
}
