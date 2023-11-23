use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CHECK_ARGS: Regex =
        Regex::new(r#"percent_change from (?P<from>\d+(?:\.\d+)?) to (?P<to>\d+(?:\.\d+)?)"#)
            .unwrap();
}

#[derive(Debug)]
pub struct ProcessedArgs {
    pub is_valid: bool,
    pub from: Option<f64>,
    pub to: Option<f64>,
}

pub fn process_args(args: Vec<String>) -> ProcessedArgs {
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
