use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let mut cnt: u32 = 0;
    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let secret_number = rand::thread_rng().gen_range(1..=100); // including 100

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                println!("got a num!");
                num
            },
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too large!"),
            Ordering::Equal => {
                println!("correct!");
                break;
            },
        }
        cnt+=1;
        if cnt >= 4 {
            println!("too many tries, exiting!");
            break;
        }
    }
}
