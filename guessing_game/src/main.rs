use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess:String = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_n) => {
            //println!("{} bytes read", n);
            println!("{}", guess);
        }
        Err(error) => {
            println!("Failed to read line");
            println!("error: {}", error)
        }
    }
    //.expect("Failed to read line");
    println!("You guessed: {}", guess);

    let guess :u32 = guess.trim().parse().expect("Please, type a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => {},
        Ordering::Greater => {},
        Ordering::Equal => {},
    }
}
