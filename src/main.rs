use rand::Rng;
use std::cmp::Ordering;
use std::io; //Rng trait defines methods that random number generators implement

// cargo doc --open

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // rand::tread_rng() function, gen_range() method, defined by Rng trait: inclusive on upper and lower bounds

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut = mutable. ::new Associated function

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        //Code can't compare strings and numbers, so we need to change types.
        // match to enable error handling. If typed is not a number, we get enum Err. We then just continue.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // Trim eliminates white spaces that results from pressing enter.

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //End game on correct guess
            }
        };
    }

    //Ordering is an enum. cmp method = compare.
}
