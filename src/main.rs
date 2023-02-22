mod hackernews;
mod platsbanken;
use hackernews::hacker_news;
use platsbanken::platsbanken;
mod platsbanken_twist;
use platsbanken_twist::platsbanken_twist;
use std::io;
// mod hemnet;
// use hemnet::hemnet;

fn main() {
    loop {
        println!("Press 0 for exit");
        println!("1. Hackernews");
        println!("2. Platsbanken");
        println!("3. Platsbanken with a twist");
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
                println!("Enter job...");
                let mut keyword = String::new();

                io::stdin()
                    .read_line(&mut keyword)
                    .expect("Failed to read input");
                println!("Accessing platsbanken...");
                platsbanken(keyword);
            }
            3 => {
                // make a vector of strings
                let mut includes: Vec<String> = Vec::new();

                println!("Enter job...");
                let mut keyword = String::new();

                io::stdin()
                    .read_line(&mut keyword)
                    .expect("Failed to read input");

                loop {
                    println!("Enter a word that must be included in the job description, leave blank if none or done...");
                    // save the input to include string and trim it
                    let mut include = String::new();

                    io::stdin()
                        .read_line(&mut include)
                        .expect("Failed to read input");
                    // save the input to include string and trim it
                    if include.trim().is_empty() {
                        break;
                    }

                    includes.push(include.trim().to_string());
                }

                println!("Accessing platsbanken...");
                platsbanken_twist(keyword, includes);
                //hemnet();
                //println!("Accessing Hemnet...");
            }
            _ => println!("Invalid input"),
        }
    }
}
