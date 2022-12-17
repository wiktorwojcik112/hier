use std::{fs, io, panic, thread};
use std::io::Write;
use std::process::exit;
use crate::{Environment, Parser, Tokenizer};
use crate::expression::Expression;
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

        environment.interpret();
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
        environment.interpret()
    }

    pub fn repl(&mut self) -> ! {
        let mut repl_environment = Environment::new(true);

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

            let code = if let Expression::BLOCK(code) = parser.code {
                code
            } else {
                vec![parser.code]
            };

            let environment = repl_environment.clone();

            let value = thread::spawn(move || {
                let mut environment = environment.clone();
                let value = environment.interpret_block(code);
                (value, environment.values)
            });

            let current_hook = panic::take_hook();

            panic::set_hook(Box::new(|_info| {
                // Do nothing.
            }));

            match value.join() {
                Ok((value, values)) => { println!("{}", value.text_representation()); repl_environment.values = values },
                _ => { }
            }

            panic::set_hook(current_hook);
        }
    }
}