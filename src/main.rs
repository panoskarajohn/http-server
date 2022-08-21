use std::{io, cmp::Ordering};
use rand::Rng;


fn main() {
  //guessing_game();
  //let sum = sum_multiples_of_three_and_five(1000);
  let sum = sum_fibonacci_numbers();
  println!("{sum}");
  
}

///
/// A small guesing game for you to play
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


/// Returns the sum of all even fib numbers up to 4_000_000 fib number
fn sum_fibonacci_numbers() -> u32
{
    let mut a = 0;
    let mut b = 1;
    let mut sum = 0;

    loop {
        let temp = a + b;
        a = b;
        b = temp;

        if temp > 4_000_000
        {
            break;
        }

        if b % 2 == 0
        {
            sum += b
        }
    }

    sum
}
