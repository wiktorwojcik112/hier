use crate::Environment;

use std::collections::HashMap;
use std::{fs, io};
use std::io::Write;
use std::process::exit;
use crate::{Hier, Interpreter};
use crate::interpreter::{error, warning};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use crate::expression::Expression;
use crate::value::Value;

impl Environment {
    pub fn call_addition(&mut self, arguments: Vec<Value>) -> Value {
        let mut arguments = arguments;

        let mut result_number = 0f64;
        let mut result_string = String::new();

        let mut is_number = true;

        let first = arguments.remove(0);

        if let Value::NUMBER(number) = first {
            result_number = number;
        } else if let Value::STRING(string) = first {
            is_number = false;
            result_string = string;
        } else {
            error(&format!("Argument must be a number or string in addition. Found {}.", first.text_representation()));
        }

        for argument in arguments {
            if let Value::NUMBER(number) = argument {
                if is_number {
                    result_number += number;
                } else {
                    error(&format!("Argument must be a string, but {} of type {} was found.", argument.text_representation(), argument.get_type().text_representation()))
                }
            } else if let Value::STRING(string) = argument.clone() {
                if !is_number {
                    result_string += &string;
                } else {
                    error(&format!("Argument must be a number, but {} of type {} was found.", argument.text_representation(), argument.get_type().text_representation()))
                }
            } else {
                error(&format!("Argument must be a number or string in addition. Found {}.", argument.text_representation()));
            }
        }

        if is_number { Value::NUMBER(result_number) } else { Value::STRING(result_string) }
    }

    pub fn call_subtraction(&mut self, arguments: Vec<Value>) -> Value {
        let mut arguments = arguments;
        let mut result = if let Value::NUMBER(number) = arguments.remove(0) {
            number
        } else {
            error("Argument must be a number in subtraction.")
        };

        for argument in arguments {
            if let Value::NUMBER(number) = argument {
                result -= number;
            } else {
                error(&format!("Argument must be a number in subtraction. Found {}.", argument.text_representation()));
            }
        }

        Value::NUMBER(result)
    }

    pub fn call_multiplication(&mut self, arguments: Vec<Value>) -> Value {
        let mut arguments = arguments;
        let mut result = if let Value::NUMBER(number) = arguments.remove(0) {
            number
        } else {
            error("Argument must be a number in multiplication.")
        };

        for argument in arguments {
            if let Value::NUMBER(number) = argument {
                result *= number;
            } else {
                error(&format!("Argument must be a number in multiplication. Found {}.", argument.text_representation()));
            }
        }

        Value::NUMBER(result)
    }

    pub fn call_division(&mut self, arguments: Vec<Value>) -> Value {
        let mut arguments = arguments;
        let mut result = if let Value::NUMBER(number) = arguments.remove(0) {
            number
        } else {
            error("Argument must be a number in division.")
        };

        for argument in arguments {
            if let Value::NUMBER(number) = argument {
                if number == 0.0 {
                    error("Dividing by 0 is forbidden.");
                }

                result /= number;
            } else {
                error(&format!("Argument must be a number in division. Found {}.", argument.text_representation()));
            }
        }

        Value::NUMBER(result)
    }

    pub fn call_binary(&mut self, operation: &String, arguments: Vec<Value>) -> Value {
        // Make it support many arguments.
        if arguments.len() != 2 {
            error("Binary operations require only 2 operands");
        }

        match &operation as &str {
            "%" => if let Value::NUMBER(number1) = arguments[0].clone() {
                if let Value::NUMBER(number2) = arguments[1].clone() {
                    Value::NUMBER(((number1 as i64) % (number2 as i64)) as f64)
                } else {
                    error("Modulo operation requires 2 number arguments.");
                }
            } else {
                error("Modulo operation requires 2 number arguments.");
            },
            "??" => if arguments[0] != Value::NULL { arguments[0].clone() } else { arguments[1].clone() },
            "==" => Value::BOOL(arguments[0] == arguments[1]),
            "is" => {
                if let Value::TYPE(a_type) = arguments[1].clone() {
                    Value::BOOL(arguments[0].get_type() == a_type)
                } else {
                    error("Is operation requires second argument to be value type.")
                }
            },
            "!=" => Value::BOOL(arguments[0] != arguments[1]),
            "<" => {
                if let Value::NUMBER(number0) = arguments[0] {
                    if let Value::NUMBER(number1) = arguments[1] {
                        Value::BOOL(number0 < number1)
                    } else {
                        error("< comparison operands must be numbers.")
                    }
                } else {
                    error("< comparison operands must be numbers.")
                }
            },
            ">" => {
                if let Value::NUMBER(number0) = arguments[0] {
                    if let Value::NUMBER(number1) = arguments[1] {
                        Value::BOOL(number0 > number1)
                    } else {
                        error("> comparison operands must be numbers.")
                    }
                } else {
                    error("> comparison operands must be numbers.")
                }
            },
            "<=" => {
                if let Value::NUMBER(number0) = arguments[0] {
                    if let Value::NUMBER(number1) = arguments[1] {
                        Value::BOOL(number0 <= number1)
                    } else {
                        error("<= comparison operands must be numbers.")
                    }
                } else {
                    error("<= comparison operands must be numbers.")
                }
            },
            ">=" => {
                if let Value::NUMBER(number0) = arguments[0] {
                    if let Value::NUMBER(number1) = arguments[1] {
                        Value::BOOL(number0 >= number1)
                    } else {
                        error(">= comparison operands must be numbers.")
                    }
                } else {
                    error(">= comparison operands must be numbers.")
                }
            },
            _ => Value::NULL
        }
    }

    pub fn call_logical(&mut self, operation: &String, arguments: Vec<Value>) -> Value {
        if arguments.len() == 0 {
            return Value::BOOL(true);
        }

        for argument in arguments {
            if let Value::BOOL(value) = argument {
                if operation == "&&" {
                    if !value {
                        return Value::BOOL(false);
                    }
                } else { // ||
                    if value {
                        return Value::BOOL(true);
                    }
                };
            } else {
                error("Operands of logical operations must be booleans or boolean expressions.")
            }
        }

        if operation == "&&" {
            Value::BOOL(true)
        } else { // ||
            Value::BOOL(false)
        }
    }

    pub fn call_if(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 && arguments.len() != 3 {
            error("If must have only 2 or 3 arguments: condition and block (optionally else block).");
        }

        let condition = if let Value::BOOL(condition) = arguments[0] {
            condition
        } else {
            error("If's condition must evaluate to a boolean.");
        };

        let mut environment = self.child();
        let mut interpreter = Interpreter::new(Expression::NUMBER(0f64), &mut environment);

        if condition {
            return if let Value::BLOCK(block) = arguments[1].clone() {
                let value = interpreter.interpret_block(block);
                self.restore(environment);
                value
            } else {
                arguments[1].clone()
            }
        } else if arguments.len() == 3 {
            return if let Value::BLOCK(block) = arguments[2].clone() {
                let value = interpreter.interpret_block(block);
                self.restore(environment);
                value
            } else {
                arguments[2].clone()
            }
        }

        Value::NULL
    }

    pub fn call_while(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 3 {
            error("While must have 3 arguments: initialization block, condition block and execution block.");
        }

        let mut environment = self.child();
        let mut interpreter = Interpreter::new(Expression::NUMBER(0f64), &mut environment);

        if let Value::BLOCK(block) = arguments[0].clone() {
            interpreter.interpret_block(block);
        } else {
            error("While's first argument must be a block.");
        }

        if let Value::BLOCK(_) = arguments[1] { } else {
            error("While's second argument must be a block.");
        }

        if let Value::BLOCK(block) = arguments[2].clone() {
            loop {
                let condition = if let Value::BLOCK(condition_block) = arguments[1].clone() {
                    if let Value::BOOL(condition) = interpreter.interpret_block(condition_block) {
                        condition
                    } else {
                        error("While's condition must return a boolean (boolean must be the last expression's result).");
                    }
                } else {
                    error("While's condition must be a condition block returning a boolean (boolean must be the last expression's result).");
                };

                if !condition {
                    break;
                }

                if let Value::ERROR(error_message) = interpreter.interpret_block(block.clone()) {
                    if error_message == "LoopExit".to_string() {
                        break;
                    }
                }
            }
        }

        self.restore(environment);

        Value::NULL
    }

    pub fn call_try(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            error("Try must have 2 arguments: a value and execution block.");
        }

        let mut environment = self.child();
        let mut interpreter = Interpreter::new(Expression::NUMBER(0f64), &mut environment);

        let result = if let Value::ERROR(error_messsage) = arguments[0].clone() {
            if let Value::BLOCK(block) = arguments[1].clone() {
                interpreter.environment.set("error".to_string(), Value::STRING(error_messsage));
                interpreter.interpret_block(block.clone())
            } else {
                error("Try's second argument must be a block.");
            }
        } else {
            arguments[0].clone()
        };

        self.restore(environment);

        result
    }

    pub fn call_for(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            error("For must have 2 arguments: a list or a string and execution block.");
        }

        let mut environment = self.child();
        let mut interpreter = Interpreter::new(Expression::NUMBER(0f64), &mut environment);

        if let Value::LIST(list) = arguments[0].clone() {
            if let Value::BLOCK(block) = arguments[1].clone() {
                for element in list {
                    interpreter.environment.set("element".to_string(), element);
                    if let Value::ERROR(error_message) = interpreter.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            break;
                        }
                    }
                }
            } else {
                error("For's second argument must be a block.");
            }
        } else if let Value::STRING(string) = arguments[0].clone() {
            if let Value::BLOCK(block) = arguments[1].clone() {
                for element in string.chars() {
                    interpreter.environment.set("element".to_string(), Value::STRING(element.to_string()));
                    if let Value::ERROR(error_message) = interpreter.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            break;
                        }
                    }
                }
            } else {
                error("For's second argument must be a block.");
            }
        } else if let Value::TABLE(table) = arguments[0].clone() {
            if let Value::BLOCK(block) = arguments[1].clone() {
                for (key, value) in table.iter() {
                    interpreter.environment.set("element".to_string(), Value::KEY_VALUE(key.to_string(), Box::new(value.clone())));
                    if let Value::ERROR(error_message) = interpreter.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            break;
                        }
                    }
                }
            } else {
                error("For's second argument must be a block.");
            }
        } else {
            error("For's first argument must be a list.");
        };

        self.restore(environment);

        Value::NULL
    }

    pub fn call_repeat(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 && arguments.len() != 1 {
            error("Repeat must have only 2 arguments: a number (optional) and execution block.");
        }

        let mut environment = self.child();
        let mut interpreter = Interpreter::new(Expression::NUMBER(0f64), &mut environment);

        if arguments.len() == 2 {
            let repetitions = if let Value::NUMBER(number) = arguments[0].clone() {
                if number < 1f64 {
                    error("Repeat's first argument must be a number greater than 0.");
                }
                number as i64
            } else {
                error("Repeat's first argument must be a number.");
            };

            if let Value::BLOCK(block) = arguments[1].clone() {
                for _ in 0..repetitions {
                    if let Value::ERROR(error_message) = interpreter.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            break;
                        }
                    }
                }
            }
        } else {
            loop {
                if let Value::BLOCK(block) = arguments[0].clone() {
                    if let Value::ERROR(error_message) = interpreter.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            break;
                        }
                    }
                }
            }
        }

        self.restore(environment);

        Value::NULL
    }

    pub fn call_run(&mut self, arguments: Vec<Value>) -> Value {
        let mut last_result = Value::NULL;

        let mut interpreter = Interpreter::new(Expression::NUMBER(0f64), self);

        for argument in arguments {
            if let Value::BLOCK(block) = argument {
                last_result = interpreter.interpret_block(block);
            } else {
                last_result = argument;
            }
        }

        last_result
    }

    pub fn call_print(&mut self, arguments: Vec<Value>) -> Value {
        for argument in arguments {
            print!("{}", argument.text_representation());
        }

        std::io::stdout().flush().expect("Failed to flush stdout.");

        Value::NULL
    }

    pub fn call_println(&mut self, arguments: Vec<Value>) -> Value {
        for argument in arguments {
            print!("{}", argument.text_representation());
        }

        print!("\n");

        Value::NULL
    }

    pub fn call_list(&mut self, arguments: Vec<Value>) -> Value {
        Value::LIST(arguments)
    }

    pub fn call_write(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() == 2{
            if let Value::STRING(path) = arguments[0].clone() {
                if let Value::STRING(contents) = arguments[1].clone() {
                    match std::fs::write(path, contents.as_bytes()) {
                        Ok(_bytes) => Value::STRING(contents),
                        Err(error) => { warning(&format!("Failed to write to file: {}", error)); Value::NULL }
                    }
                } else {
                    error("Write operation requires second argument to be a string to write.");
                }
            } else {
                error("Write operation requires first argument to be a string path to file.");
            }
        } else {
            error("Write operation requires 2 arguments: path string and contents string.");
        }
    }

    pub fn call_read(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() == 0 {
            let mut line = String::new();
            if let Err(error_message) = io::stdin().read_line(&mut line) {
                error(&format!("Failed to read line: {}.", error_message));
            };
            if 1 < line.len() {
                line.remove(line.len() - 1);
            } else {
                line = "".to_string();
            }

            Value::STRING(line)
        } else if arguments.len() == 1 {
            if let Value::STRING(path) = arguments[0].clone() {
                match fs::read_to_string(path) {
                    Ok(contents) => Value::STRING(contents),
                    Err(error) => Value::ERROR(error.to_string())
                }
            } else {
                error("Read operation requires first argument to be a string path to file.");
            }
        } else {
            error("Read operation requires 0 or 1 arguments (a path).");
        }
    }

    pub fn call_negate(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            error("Negation requires 1 boolean argument.");
        }

        if let Value::BOOL(boolean) = arguments[0] {
            Value::BOOL(!boolean)
        } else {
            error("Negation requires 1 boolean argument.");
        }
    }

    pub fn call_number(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            error("Number conversion requires 1 argument.");
        }

        if let Value::STRING(string) = arguments[0].clone() {
            let string = if !string.contains('.') {
                string + ".0"
            } else {
                string
            };

            let number = string.parse::<f64>();

            match number {
                Ok(number) => Value::NUMBER(number),
                Err(err) => {
                    println!("Warning: Failed to convert number {} due to an error: {}. Returning 0.", string, err);
                    Value::NUMBER(0f64)
                }
            }
        } else {
            println!("Warning: Failed to convert to number from {}, because it is an unsupported type. Returning 0.", arguments[0].clone().get_type().text_representation());
            Value::NULL
        }
    }

    pub fn call_table(&mut self, arguments: Vec<Value>) -> Value {
        let mut table: HashMap<String, Value> = HashMap::new();

        for argument in arguments {
            if let Value::KEY_VALUE(key, value) = argument {
                table.insert(key, *value);
            } else {
                error(&format!("Table operation's all arguments must be key-values, but {} was found.", argument.text_representation()));
            }
        }

        Value::TABLE(table)
    }

    pub fn call_string(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            error("String conversion requires 1 argument.");
        }

        Value::STRING(arguments[0].text_representation())
    }

    pub fn call_length(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            error("Length operation requires 1 argument that is an array (list or string).");
        }

        if let Value::LIST(list) = arguments[0].clone() {
            Value::NUMBER(list.len() as f64)
        } else if let Value::STRING(string) = arguments[0].clone() {
            Value::NUMBER(string.len() as f64)
        } else {
            error("Length operation requires 1 argument that is an array (list or string).");
        }
    }

    pub fn call_remove(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() == 1 {
            if let Value::LIST(list) = arguments[0].clone() {
                let mut list = list;
                list.remove(list.len() - 1);
                Value::LIST(list)
            } else if let Value::STRING(string) = arguments[0].clone() {
                let mut string = string;
                string.remove(string.len() - 1);
                Value::STRING(string)
            } else {
                error("Remove operation requires first argument to be an array (list or string).");
            }
        } else if arguments.len() == 2 {
            if let Value::NUMBER(index) = arguments[1].clone() {
                let index = index as usize;

                if let Value::LIST(list) = arguments[0].clone() {
                    let mut list = list;
                    list.remove(index);
                    Value::LIST(list)
                } else if let Value::STRING(string) = arguments[0].clone() {
                    let mut string = string;
                    string.remove(index);
                    Value::STRING(string)
                } else {
                    error("Remove operation requires first argument to be an array (list or string).");
                }
            } else {
                error("Remove operation requires second argument to be a number.");
            }
        } else {
            error("Remove operation requires 1 or 2 arguments: an array (list or string) and index (optional, if none, operate on last element).");
        }
    }

    pub fn call_replace(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 3 {
            error("Replace operation requires 3 arguments: an array (list or string), index and value.");
        }
        if let Value::NUMBER(index) = arguments[1].clone() {
            let index = index as usize;

            if let Value::LIST(list) = arguments[0].clone() {
                let mut list = list;
                list[index] = arguments[2].clone();
                Value::LIST(list)
            } else if let Value::STRING(string) = arguments[0].clone() {
                if let Value::STRING(new) = arguments[2].clone() {
                    let mut string = string;
                    string.replace_range(index..(index + 1), &new);
                    Value::STRING(string)
                } else {
                    error("Replace operation requires third argument to be an string if array is a string.");
                }
            } else {
                error("Replace operation requires first argument to be an array (list or string).");
            }
        } else {
            error("Replace operation requires second argument to be a number.");
        }
    }

    pub fn call_insert(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() == 2 {
            if let Value::LIST(list) = arguments[0].clone() {
                let mut list = list;
                list.push(arguments[1].clone());
                Value::LIST(list)
            } else if let Value::STRING(string) = arguments[0].clone() {
                if let Value::STRING(appended) = arguments[1].clone() {
                    let mut string = string;
                    string.push_str(&appended);
                    Value::STRING(string)
                } else {
                    error("Insert operation requires second argument to be a string when array is a string.");
                }
            } else {
                error("Insert operation requires first argument to be an array (list or string).");
            }
        } else if arguments.len() == 3 {
            if let Value::NUMBER(index) = arguments[2].clone() {
                let index = index as usize;

                if let Value::LIST(list) = arguments[0].clone() {
                    let mut list = list;
                    list.insert(index, arguments[1].clone());
                    Value::LIST(list)
                } else if let Value::STRING(string) = arguments[0].clone() {
                    if let Value::STRING(appended) = arguments[1].clone() {
                        let mut string = string;
                        string.insert_str(index, &appended);
                        Value::STRING(string)
                    } else {
                        error("Insert operation requires second argument to be a string when array is a string.");
                    }
                } else {
                    error("Insert operation requires first argument to be an array (list or string).");
                }
            } else {
                error("Insert operation requires third argument to be a number.");
            }
        } else {
            error("Insert operation requires 2 or 3 arguments: an array (list or string), value and index (optional, if none, operate on last element).");
        }
    }

    pub fn call_set(&mut self, key: String, value: Value) -> Value {
        self.set(key, value.clone());
        value
    }

    pub fn call_time(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 0 {
            error("Time operation requires 0 arguments.");
        }

        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Value::NUMBER(n.as_secs() as f64),
            Err(_) => error("System time is before Unix epoch."),
        }
    }

    pub fn call_break(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 0 {
            error("Break operation requires 0 arguments.");
        }

        Value::ERROR("LoopExit".to_string())
    }

    pub fn call_round(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            error("Round operation requires 1 number argument.");
        }

        if let Value::NUMBER(number) = arguments[0] {
            Value::NUMBER(number as i64 as f64)
        } else {
            error("Round operation requires a number argument.");
        }
    }

    pub fn call_error(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            error("Error operation requires 1 argument.");
        }

        Value::ERROR(arguments[0].clone().text_representation())
    }

    pub fn call_panic(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            error("Panic operation requires 1 argument.");
        }

        eprintln!("! Panic: {}", arguments[0].clone().text_representation());
        exit(1);
    }

    pub fn call_eval(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            error("Evaluate operation requires 1 string argument.");
        }

        if let Value::STRING(code) = arguments[0].clone() {
            let mut hier = Hier::new();
            hier.run(code)
        } else {
            error("Evaluate operation requires a string argument.");
        }
    }

    pub fn call_cmd(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            error("Cmd operation requires at least 1 string argument.");
        }

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
            error("Cmd operation requires a string argument.");
        }
    }

    pub fn call_random(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            error("Random operation requires 2 number arguments. First smaller than second.");
        }

        if let Value::NUMBER(first) = arguments[0] {
            if let Value::NUMBER(second) = arguments[1] {
                if first >= second {
                    error("Random operation's first argument must be smaller than second.");
                }

                let mut rng = rand::thread_rng();

                Value::NUMBER(rng.gen_range(first, second) as f64)
            } else {
                error("Random operation's second argument must be a number.");
            }
        } else {
            error("Random operation's first argument must be a number.");
        }
    }

    pub fn call_get(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 && arguments.len() != 1 {
            error("Get operation requires max 2 arguments: object and key (number or string, optional).");
        }

        if arguments.len() == 1 {
            return arguments[0].clone();
        }

        if let Value::STRING(property) = arguments[1].clone() {
            match arguments[0].clone() {
                Value::KEY_VALUE(key, value) => {
                    if property == "value" {
                        *value
                    } else if property == "key" {
                        Value::STRING(key)
                    } else {
                        Value::NULL
                    }
                },
                Value::TABLE(table) => {
                    if table.contains_key(&property) {
                        table.get(&property).unwrap().clone()
                    } else {
                        Value::NULL
                    }
                },
                _ => Value::NULL
            }
        } else if let Value::NUMBER(index) = arguments[1] {
            match arguments[0].clone() {
                Value::LIST(value) => {
                    if index < 0f64 || value.len() <= index as usize {
                        error(&format!("Index {} is out of bounds ({} elements).", index, value.len()));
                    }

                    value[index as usize].clone()
                },
                Value::STRING(value) => {
                    if index < 0f64 || value.len() <= index as usize {
                        error(&format!("Index {} is out of bounds ({} elements).", index, value.len()));
                    }
                    Value::STRING(value.chars().nth(index as usize).clone().unwrap_or(' ').to_string())
                },
                _ => if index == 0f64 { arguments[0].clone() } else { Value::NULL },
            }
        } else {
            error("Get operation requires second arguments to be a number or string.");
        }
    }
}