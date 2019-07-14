use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess: String = String::new();
        println!("Please input your guess.");

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

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
