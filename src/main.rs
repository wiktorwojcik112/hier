mod functions;
mod hier;

extern crate core;

use std::{env, fs, io, panic};
use std::env::current_dir;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use hier::environment::{Environment, VariableId};
use hier::expression::Expression;
use hier::hier::Hier;
use hier::parser::Parser;
use hier::tokenizer::Tokenizer;
use hier::value::Value;
use functions::*;

fn print_usage() {
    eprintln!("Usage: hier <command>");
    eprintln!("Commands:");
    eprintln!(" repl - Runs REPL. Can be omitted by running without arguments.");
    eprintln!(" file - Runs contents of file. Can be omitted by running with only path.");
    eprintln!(" run  - Runs a string.");
}

fn module_reader(path: String) -> String {
    fs::read_to_string(path.clone())
        .expect(&format!("Unable to read the file: {}", path))
}

fn exit_handler() -> ! {
    exit(0)
}

fn add_defaults(hier: &mut Hier) {
    hier.add_variable("cwd".to_string(), match current_dir() {
        Ok(path) => Value::STRING(path.to_str().unwrap().to_string()),
        Err(_) => Value::NULL,
    });

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    hier.add_variable("args".to_string(), Value::LIST(args.iter().map(|arg| Value::STRING(arg.to_string())).collect()));

    hier.add_function("time".to_string(), 0, time_function);
    hier.add_function("rand".to_string(), 2, rand_function);
    hier.add_function("cmd".to_string(), 1, cmd_function);
    hier.add_function("write".to_string(), 2, write_function);
    hier.add_function("file".to_string(), 1, file_function);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut hier = Hier::new("./repl".to_string(), module_reader, exit_handler);
        add_defaults(&mut hier);
        repl();
    } else if args.len() == 2 {
        let path = args[1].clone();
        let contents = fs::read_to_string(path.clone())
            .expect("Unable to read the file.");
        let full_path = fs::canonicalize(PathBuf::from(path)).expect("Unable to resolve file.").to_str().unwrap().to_string();
        let mut hier = Hier::new(full_path, module_reader, exit_handler);
        add_defaults(&mut hier);
        hier.run(contents);
    } else if args.len() == 3 {
        match &args[1] as &str {
            "file" => {
                let path = args[2].clone();
                let contents = fs::read_to_string(path.clone())
                    .expect("Unable to read the file.");
                let full_path = fs::canonicalize(PathBuf::from(path)).expect("Unable to resolve file.").to_str().unwrap().to_string();
                let mut hier = Hier::new(full_path, module_reader, exit_handler);
                add_defaults(&mut hier);
                hier.run(contents);
            },
            "run" => {
                let mut hier = Hier::new("./code".to_string(), module_reader, exit_handler);
                add_defaults(&mut hier);
                hier.run(args[2].clone());
            },
            "repl" => {
                let mut hier = Hier::new("./repl".to_string(), module_reader, exit_handler);
                add_defaults(&mut hier);
                repl()
            },
            _ => { print_usage(); exit(1) }
        }
    } else {
        print_usage();
        exit(1)
    }
}

fn repl() -> ! {
    let mut repl_environment = Environment::new(true, "./repl".to_string(), module_reader, exit_handler);

    repl_environment.values.insert(VariableId(0, "cwd".to_string()), match current_dir() {
        Ok(path) => Value::STRING(path.to_str().unwrap().to_string()),
        Err(_) => Value::NULL,
    });

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    repl_environment.values.insert(VariableId(0, "args".to_string()), Value::LIST(args.iter().map(|arg| Value::STRING(arg.to_string())).collect()));

    repl_environment.values.insert(VariableId(0, "time".to_string()), Value::NATIVE_FUNCTION(time_function, 0));
    repl_environment.values.insert(VariableId(0, "rand".to_string()), Value::NATIVE_FUNCTION(rand_function, 2));
    repl_environment.values.insert(VariableId(0, "cmd".to_string()), Value::NATIVE_FUNCTION(cmd_function, 1));
    repl_environment.values.insert(VariableId(0, "write".to_string()), Value::NATIVE_FUNCTION(write_function, 2));
    repl_environment.values.insert(VariableId(0, "file".to_string()), Value::NATIVE_FUNCTION(file_function, 1));

    loop {
        print!("> ");
        std::io::stdout().flush().expect("Failed to flush stdout.");

        let mut line = String::new();
        if let Err(error) = io::stdin().read_line(&mut line) {
            eprintln!("Failed to read line: {}.", error);
            exit_handler();
        };

        if line == "(exit)\n" || line == "exit\n" {
            exit_handler();
        }

        let mut tokenizer = Tokenizer::new(line);

        tokenizer.module_name = "REPL".to_string();

        if tokenizer.tokenize_code() {
            eprintln!("Failed.");
            continue;
        }

        let mut parser = Parser::new(tokenizer.tokens, module_reader, exit_handler);

        if parser.parse() {
            eprintln!("Failed.");
            continue;
        }

        let code = if let Expression::BLOCK(code, _) = parser.code {
            code
        } else {
            vec![parser.code]
        };

        let environment = repl_environment.clone();

        let current_hook = panic::take_hook();

        panic::set_hook(Box::new(|_info| {
            // Do nothing.
        }));

        let value = panic::catch_unwind(move || {
            let mut environment = environment.clone();
            let value = environment.interpret_block(code);
            (value, environment.values)
        });

        panic::set_hook(current_hook);

        match value {
            Ok((value, values)) => { println!("{}", value.text_representation()); repl_environment.values = values },
            _ => { }
        }
    }
}