#![no_std]

fn main() {
    println!("Hello, world!");

    println!(first_word("lalala fofofo dododo"));

    fizz_buzz();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn fizz_buzz() {
    for number in 1..=100 {
        match (number % 3, number % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", number),
        }
    }
}