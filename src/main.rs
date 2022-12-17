extern crate core;

use std::env;
use std::process::exit;
use crate::environment::Environment;
use crate::hier::Hier;
use crate::location::Location;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

mod tokenizer;
mod parser;
mod interpreter;
mod environment;
mod hier;
mod native_functions;
mod types;
mod value;
mod token;
mod location;
mod expression;

fn report(error: &str, location: Location) {
    eprintln!("! [{}:{}] in {}: {}", location.line_number, location.offset, location.module, error);
}

fn print_usage() {
    eprintln!("Usage: hier <command>");
    eprintln!("Commands:");
    eprintln!(" repl - Runs REPL. Can be omitted by running without arguments.");
    eprintln!(" file - Runs contents of file. Can be omitted by running with only path.");
    eprintln!(" run  - Runs a string.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut hier = Hier::new();
        hier.repl();
    } else if args.len() == 2 {
        let path = args[1].clone();
        let mut hier = Hier::new();
        hier.run_file(path);
    } else if args.len() == 3 {
        match &args[1] as &str {
            "file" => {
                let path = args[1].clone();
                let mut hier = Hier::new();
                hier.run_file(path);
            },
            "run" => {
                let mut hier = Hier::new();
                hier.run(args[2].clone());
            },
            "repl" => {
                let mut hier = Hier::new();
                hier.repl();
            },
            _ => { print_usage(); exit(1) }
        }
    } else {
        print_usage();
        exit(1)
    }
}