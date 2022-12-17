use std::{fs, io};
use std::io::Write;
use std::process::exit;
use crate::{Environment, Parser, Tokenizer};
use crate::value::Value;

pub struct Hier { }

impl Hier {
    pub fn new() -> Self {
        Self { }
    }

    pub fn run_file(&mut self, path: String) {
        let contents = fs::read_to_string(path)
            .expect("Unable to read the file.");

        let mut tokenizer = Tokenizer::new(contents);

        if tokenizer.tokenize() {
            eprintln!("Failed.");
            exit(1);
        }

        let mut parser = Parser::new(tokenizer.tokens);

        if parser.parse() {
            eprintln!("Failed.");
            exit(1);
        }

        let mut environment = Environment::new_with_code(parser.code, false);

        environment.file_interpret();
    }

    pub fn run(&mut self, code: String) -> Value {
        let mut tokenizer = Tokenizer::new(code);

        if tokenizer.tokenize() {
            eprintln!("Failed.");
            exit(1);
        }

        let mut parser = Parser::new(tokenizer.tokens);

        if parser.parse() {
            eprintln!("Failed.");
            exit(1);
        }

        let mut environment = Environment::new_with_code(parser.code, false);
        environment.direct_interpret()
    }

    pub fn repl(&mut self) -> ! {
        let mut environment = Environment::new(true);

        loop {
            print!("> ");
            std::io::stdout().flush().expect("Failed to flush stdout.");

            let mut line = String::new();
            if let Err(error) = io::stdin().read_line(&mut line) {
                eprintln!("Failed to read line: {}.", error);
                exit(1);
            };

            if line == "(exit)\n" {
                exit(0);
            }

            let mut tokenizer = Tokenizer::new(line);

            tokenizer.module_name = "REPL".to_string();

            if tokenizer.tokenize() {
                eprintln!("Failed.");
                continue;
            }

            let mut parser = Parser::new(tokenizer.tokens);

            if parser.parse() {
                eprintln!("Failed.");
                continue;
            }
            println!("{}", environment.interpret(parser.code).text_representation());
        }
    }
}