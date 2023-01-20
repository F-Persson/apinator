mod hackernews;
mod platsbanken;
use hackernews::hacker_news;
use platsbanken::platsbanken;
use std::io;
mod hemnet;
use hemnet::hemnet;

fn main() {
    loop {
        println!("Press 0 for exit");
        println!("1. Hackernews");
        println!("2. Hemnet");
        println!("3. Platsbanken");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: i32 = input.trim().parse().expect("Please type a number!");

        match input {
            0 => {
                println!("Exiting...");
                break;
            }
            1 => {
                println!("Accessing Hackernews...");
                hacker_news();
            }
            2 => {
                println!("Accessing Hemnet...");
                hemnet();
            }
            3 => {
                println!("Enter job...");
                let mut keyword = String::new();

                io::stdin()
                    .read_line(&mut keyword)
                    .expect("Failed to read input");
                println!("Accessing platsbanken...");
                platsbanken(keyword);
            }
            _ => println!("Invalid input"),
        }
    }
}
