mod args_processing;
mod program_options;
mod running;
mod utils;

use program_options::ProgramOptions;
use running::{run_menu, run_with_args};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let program_opts = if args.len() == 1 {
        run_menu()
    } else {
        run_with_args(args)
    };

    match program_opts {
        Ok(opts) => display_change(opts),
        Err(ref message) => handle_err(message),
    }
}

fn compute_percent_change(from: f64, to: f64) -> f64 {
    (to - from) / from * 100.0
}

fn display_change(opts: ProgramOptions) {
    println!(
        "Percent change is {:.*}%",
        opts.precision,
        compute_percent_change(opts.from, opts.to)
    );
}

fn handle_err(message: &str) {
    eprintln!("{}", message);
}
