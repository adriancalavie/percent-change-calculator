use lazy_static::lazy_static;
use regex::Regex;

const PATTERN: &str = r#"percent_change from (?P<from>\d+(?:\.\d+)?) to (?P<to>\d+(?:\.\d+)?)"#;
// while I would've liked to count the words from the pattern, that's impossible to achieve at compile-time
const WORD_COUNT: usize = 6;

lazy_static! {
    static ref CHECK_ARGS: Regex = Regex::new(PATTERN).unwrap();
}

#[derive(Debug)]
pub struct ProcessedArgs {
    pub is_valid: bool,
    pub from: Option<f64>,
    pub to: Option<f64>,
}

pub fn process_args(args: Vec<String>) -> Result<ProcessedArgs, &'static str> {
    let command = args.join(" ");
    let is_valid = length_valid(args) && structure_valid(&command);

    if !is_valid {
        return Ok(ProcessedArgs {
            is_valid,
            from: None,
            to: None,
        });
    }

    let parser = CmdParser::new(&command);

    match parser {
        Ok(parser) => Ok(ProcessedArgs {
            is_valid,
            from: parser.parse_float("from"),
            to: parser.parse_float("to"),
        }),
        Err(message) => Err(message),
    }
}

fn length_valid(args: Vec<String>) -> bool {
    args.len() == WORD_COUNT
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

    fn new(command: &'a str) -> Result<CmdParser<'a>, &'static str> {
        match CHECK_ARGS.captures(command) {
            Some(caps) => Ok(CmdParser { caps }),
            None => Err("Regex match failed"),
        }
    }
}
