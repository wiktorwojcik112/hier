use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::hier::environment::Environment;
use crate::hier::value::Value;
use rand::Rng;

pub fn time_function(environment: &mut Environment, _arguments: Vec<Value>) -> Value {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => Value::NUMBER(n.as_secs() as f64),
        Err(_) => environment.error("System time is before Unix epoch."),
    }
}

pub fn write_function(environment: &mut Environment, arguments: Vec<Value>) -> Value {
    if let Value::STRING(path) = arguments[0].clone() {
        if let Value::STRING(contents) = arguments[1].clone() {
            match std::fs::write(path, contents.as_bytes()) {
                Ok(_bytes) => Value::STRING(contents),
                Err(error) => { println!("{}", &format!("Failed to write to file: {}", error)); Value::NULL }
            }
        } else {
            environment.error("Write operation requires second argument to be a string to write.");
        }
    } else {
        environment.error("Write operation requires first argument to be a string path to file.");
    }
}

pub fn file_function(environment: &mut Environment, arguments: Vec<Value>) -> Value {
    if let Value::STRING(path) = arguments[0].clone() {
        match fs::read_to_string(path) {
            Ok(contents) => Value::STRING(contents),
            Err(error) => Value::ERROR(error.to_string())
        }
    } else {
        environment.error("Read operation requires first argument to be a string path to file.");
    }
}

pub fn cmd_function(environment: &mut Environment, arguments: Vec<Value>) -> Value {
    let mut args: Vec<String> = arguments.iter().map(|value| value.clone().text_representation()).collect();
    args.remove(0);

    if let Value::STRING(command) = arguments[0].clone() {
        let process = match std::process::Command::new(command)
            .args(args)
            .spawn() {
            Ok(process) => process,
            Err(error) => return Value::ERROR(error.to_string()),
        };

        let output = match process.wait_with_output() {
            Ok(output)  => output,
            Err(error) => return Value::ERROR(error.to_string()),
        };

        let string_output = match std::string::String::from_utf8(output.stdout) {
            Ok(string_output)  => string_output,
            Err(error) => return Value::ERROR(error.to_string()),
        };

        Value::STRING(string_output)
    } else {
        environment.error("Cmd operation requires a string argument.");
    }
}

pub fn rand_function(environment: &mut Environment, arguments: Vec<Value>) -> Value {
    if let Value::NUMBER(first) = arguments[0] {
        if let Value::NUMBER(second) = arguments[1] {
            if first >= second {
                environment.error("Random operation's first argument must be smaller than second.");
            }

            let mut rng = rand::thread_rng();

            Value::NUMBER(rng.gen_range(first, second) as f64)
        } else {
            environment.error("Random operation's second argument must be a number.");
        }
    } else {
        environment.error("Random operation's first argument must be a number.");
    }
}