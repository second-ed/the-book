use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("type a number smart ass");
                continue;
            },
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small"),
            Ordering::Greater => println!("{guess} is too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }
}

