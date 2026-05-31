use std::env;
use std::io;
use std::process;
use std::time::Instant;

use kaadivisors::get_divisors;

fn main() {
    println!("This program returns divisors of a number");

    run(env::args());
}

fn run(mut env_args: impl Iterator<Item = String>) {
    env_args.next().unwrap(); // skip first element

    let number = match env_args.next() {
        Some(arg) => match arg.trim().parse::<u128>() {
            Ok(number) => number,
            Err(_) => {
                eprintln!("Please, enter an integer in range [0; 2^128)!");
                process::exit(1);
            }
        },
        None => ask_number(),
    };

    println!("----------");

    let now = Instant::now();
    let result = get_divisors(number);

    print_divisors(&result);
    println!("----------\nFinished: {:?}", now.elapsed());
}

fn ask_number() -> u128 {
    println!("Enter the number:");

    let mut num = String::new();
    match io::stdin().read_line(&mut num) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error while reading the number: {e}");
            process::exit(1);
        }
    }

    match num.trim().parse::<u128>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Please, enter an integer in range [0; 2^128)!");
            process::exit(1);
        }
    }
}

fn print_divisors(result: &Vec<(u128, u16)>) {
    for divisor in result.iter() {
        println!("{} ^{}", divisor.0, divisor.1);
    }
}
