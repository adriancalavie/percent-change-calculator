use lazy_static::lazy_static;
use regex::Regex;
use std::env;

lazy_static! {
    static ref CHECK_ARGS: Regex =
        Regex::new(r#"percent_change from (?P<from>\d+(?:\.\d+)?) to (?P<to>\d+(?:\.\d+)?)"#)
            .unwrap();
}

#[derive(Debug)]
struct ProcessedArgs {
    is_valid: bool,
    from: Option<f64>,
    to: Option<f64>,
}

fn length_valid(args: Vec<String>) -> bool {
    args.len() >= 2
}

fn structure_valid(command: &str) -> bool {
    CHECK_ARGS.is_match(command)
}

struct CmdParser<'a> {
    caps: regex::Captures<'a>,
}

impl<'a> CmdParser<'a> {
    fn parse_float(&self, token: &str) -> Option<f64> {
        self.caps
            .name(token)
            .and_then(|mat| mat.as_str().parse().ok())
    }

    fn new(command: &'a str) -> CmdParser<'a> {
        CmdParser {
            caps: CHECK_ARGS.captures(command).unwrap(),
        }
    }
}

fn process_args(args: Vec<String>) -> ProcessedArgs {
    let command = args.join(" ");
    let is_valid = length_valid(args) && structure_valid(&command);

    if !is_valid {
        return ProcessedArgs {
            is_valid,
            from: None,
            to: None,
        };
    }

    let parser = CmdParser::new(&command);

    ProcessedArgs {
        is_valid,
        from: parser.parse_float("from"),
        to: parser.parse_float("to"),
    }
}

fn percent_change(from: f64, to: f64) -> f64 {
    (to - from) / from * 100.0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let processed_args = process_args(args.clone());

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
        println!("Invalid command: Usage: \"from\" couldn't be parsed. \nUsage: percent_change from <number> to <number>");
        return;
    }

    let from = processed_args.from.unwrap();
    let to = processed_args.to.unwrap();
    
    println!("Percent change is {}%", percent_change(from, to));
}
