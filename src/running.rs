use crate::{args_processing::process_args, program_options::ProgramOptions, utils};

fn err_string(reason: &str) -> String {
    format!("Invalid command: {reason} \nUsage: percent_change from <number> to <number>")
}

pub(crate) fn run_menu() -> Result<ProgramOptions, String> {
    println!("Select from value:");
    utils::read!(from as f64);

    println!("Select to value:");
    utils::read!(to as f64);

    Ok(ProgramOptions {
        from,
        to,
        ..ProgramOptions::default()
    })
}

pub(crate) fn run_with_args(args: Vec<String>) -> Result<ProgramOptions, String> {
    let processed_args = process_args(args.clone());

    match processed_args {
        Ok(processed_args) => {
            if !processed_args.is_valid {
                return Err(err_string("Invalid structure"));
            }

            if processed_args.from.is_none() {
                return Err(err_string("\"from\" couldn't be parsed."));
            }

            if processed_args.to.is_none() {
                return Err(err_string("\"to\" couldn't be parsed."));
            }

            Ok(ProgramOptions {
                from: processed_args.from.unwrap(),
                to: processed_args.to.unwrap(),
                ..ProgramOptions::default()
            })
        }
        Err(message) => Err(err_string(message)),
    }
}
