mod args_processing;
use args_processing as argp;
use std::env;



fn percent_change(from: f64, to: f64) -> f64 {
    (to - from) / from * 100.0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let processed_args = argp::process_args(args.clone());

    if !processed_args.is_valid {
        println!(
            "Invalid command: Invalid strucutre. \nUsage: percent_change from <number> to <number>"
        );
        return;
    }

    if processed_args.from.is_none() {
        println!("Invalid command: Usage: \"from\" couldn't be parsed. \nUsage: percent_change from <number> to <number>");
        return;
    }

    if processed_args.to.is_none() {
        println!("Invalid command: Usage: \"to\" couldn't be parsed. \nUsage: percent_change from <number> to <number>");
        return;
    }

    let from = processed_args.from.unwrap();
    let to = processed_args.to.unwrap();

    println!("Percent change is {}%", percent_change(from, to));
}
