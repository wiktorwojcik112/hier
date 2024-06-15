use std::collections::HashMap;
use std::io;
use std::io::Write;
use crate::hier::environment::Environment;
use crate::hier::hier::Hier;
use crate::hier::parser::Parser;
use crate::hier::value::Value;
use crate::hier::tokenizer::Tokenizer;

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
            self.error(&format!("Argument must be a number or string in addition. Found {}.", first.text_representation()));
        }

        for argument in arguments {
            if let Value::NUMBER(number) = argument {
                if is_number {
                    result_number += number;
                } else {
                    self.error(&format!("Argument must be a string, but {} of type {} was found.", argument.text_representation(), argument.get_type().text_representation()))
                }
            } else if let Value::STRING(string) = argument.clone() {
                if !is_number {
                    result_string += &string;
                } else {
                    self.error(&format!("Argument must be a number, but {} of type {} was found.", argument.text_representation(), argument.get_type().text_representation()))
                }
            } else {
                self.error(&format!("Argument must be a number or string in addition. Found {}.", argument.text_representation()));
            }
        }

        if is_number { Value::NUMBER(result_number) } else { Value::STRING(result_string) }
    }

    pub fn call_subtraction(&mut self, arguments: Vec<Value>) -> Value {
        let mut arguments = arguments;
        let mut result = if let Value::NUMBER(number) = arguments.remove(0) {
            number
        } else {
            self.error("Argument must be a number in subtraction.")
        };

        for argument in arguments {
            if let Value::NUMBER(number) = argument {
                result -= number;
            } else {
                self.error(&format!("Argument must be a number in subtraction. Found {}.", argument.text_representation()));
            }
        }

        Value::NUMBER(result)
    }

    pub fn call_multiplication(&mut self, arguments: Vec<Value>) -> Value {
        let mut arguments = arguments;
        let mut result = if let Value::NUMBER(number) = arguments.remove(0) {
            number
        } else {
            self.error("Argument must be a number in multiplication.")
        };

        for argument in arguments {
            if let Value::NUMBER(number) = argument {
                result *= number;
            } else {
                self.error(&format!("Argument must be a number in multiplication. Found {}.", argument.text_representation()));
            }
        }

        Value::NUMBER(result)
    }

    pub fn call_division(&mut self, arguments: Vec<Value>) -> Value {
        let mut arguments = arguments;
        let mut result = if let Value::NUMBER(number) = arguments.remove(0) {
            number
        } else {
            self.error("Argument must be a number in division.")
        };

        for argument in arguments {
            if let Value::NUMBER(number) = argument {
                if number == 0.0 {
                    self.error("Dividing by 0 is forbidden.");
                }

                result /= number;
            } else {
                self.error(&format!("Argument must be a number in division. Found {}.", argument.text_representation()));
            }
        }

        Value::NUMBER(result)
    }

    pub fn call_null_coalescing(&self, arguments: Vec<Value>) -> Value {
        for argument in arguments {
            if let Value::NULL = argument {
                continue
            } else {
                return argument
            }
        }

        Value::NULL
    }

    pub fn call_modulo(&self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            self.error("Modulo requires only 2 operands");
        }

        if let Value::NUMBER(number1) = arguments[0].clone() {
            if let Value::NUMBER(number2) = arguments[1].clone() {
                Value::NUMBER(((number1 as i64) % (number2 as i64)) as f64)
            } else {
                self.error("Modulo requires 2 number arguments.");
            }
        } else {
            self.error("Modulo requires 2 number arguments.");
        }
    }

    pub fn call_is(&self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            self.error("Is requires only 2 operands");
        }

        if let Value::TYPE(a_type) = arguments[1].clone() {
            Value::BOOL(arguments[0].get_type() == a_type)
        } else {
            self.error("Is operation requires second argument to be a value type.")
        }
    }

    pub fn call_comparison(&mut self, operation: &String, arguments: Vec<Value>) -> Value {
        match &operation as &str {
            "==" => {
                for (i, argument) in arguments.iter().enumerate() {
                    if i == 0 {
                        continue
                    }

                    if argument.clone() != arguments[i - 1] {
                        return Value::BOOL(false)
                    }
                }

                return Value::BOOL(true)
            },
            "!=" => {
                for (i, argument) in arguments.iter().enumerate() {
                    if i == 0 {
                        continue
                    }

                    if argument.clone() == arguments[i - 1] {
                        return Value::BOOL(false)
                    }
                }

                return Value::BOOL(true)
            },
            "<" => {
                for (i, argument) in arguments.iter().enumerate() {
                    if i == 0 {
                        continue
                    }

                    if let Value::NUMBER(number2) = argument {
                        if let Value::NUMBER(number1) = arguments[i - 1] {
                            if number1 >= number2.clone() {
                                return Value::BOOL(false)
                            }
                        } else {
                            self.error("< comparison operands must be numbers.")
                        }
                    } else {
                        self.error("< comparison operands must be numbers.")
                    }
                }

                return Value::BOOL(true)
            },
            ">" => {
                for (i, argument) in arguments.iter().enumerate() {
                    if i == 0 {
                        continue
                    }

                    if let Value::NUMBER(number2) = argument {
                        if let Value::NUMBER(number1) = arguments[i - 1] {
                            if number1 <= number2.clone() {
                                return Value::BOOL(false)
                            }
                        } else {
                            self.error("> comparison operands must be numbers.")
                        }
                    } else {
                        self.error("> comparison operands must be numbers.")
                    }
                }

                return Value::BOOL(true)
            },
            "<=" => {
                for (i, argument) in arguments.iter().enumerate() {
                    if i == 0 {
                        continue
                    }

                    if let Value::NUMBER(number2) = argument {
                        if let Value::NUMBER(number1) = arguments[i - 1] {
                            if number1 > number2.clone() {
                                return Value::BOOL(false)
                            }
                        } else {
                            self.error("<= comparison operands must be numbers.")
                        }
                    } else {
                        self.error("<= comparison operands must be numbers.")
                    }
                }

                return Value::BOOL(true)
            },
            ">=" => {
                for (i, argument) in arguments.iter().enumerate() {
                    if i == 0 {
                        continue
                    }

                    if let Value::NUMBER(number2) = argument {
                        if let Value::NUMBER(number1) = arguments[i - 1] {
                            if number1 < number2.clone() {
                                return Value::BOOL(false)
                            }
                        } else {
                            self.error(">= comparison operands must be numbers.")
                        }
                    } else {
                        self.error(">= comparison operands must be numbers.")
                    }
                }

                return Value::BOOL(true)
            }
            _ => Value::NULL // We never reach this place, because call_function checks whether the operation is a valid one for this function.
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
                self.error("Operands of logical operations must be booleans or boolean expressions.")
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
            self.error("If must have only 2 or 3 arguments: condition and block (optionally else block).");
        }

        let condition = if let Value::BOOL(condition) = arguments[0] {
            condition
        } else {
            self.error("If's condition must evaluate to a boolean.");
        };

        self.begin_scope();

        if condition {
            return if let Value::BLOCK(block) = arguments[1].clone() {
                let value = self.interpret_block(block);
                value
            } else {
                arguments[1].clone()
            }
        } else if arguments.len() == 3 {
            return if let Value::BLOCK(block) = arguments[2].clone() {
                let value = self.interpret_block(block);
                value
            } else {
                arguments[2].clone()
            }
        }

        self.end_scope();

        Value::NULL
    }

    pub fn call_while(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            self.error("While must have 2 arguments: a condition block and an execution block.");
        }

        self.begin_scope();

        if let Value::BLOCK(_) = arguments[0] { } else {
            self.error("While's first argument must be a block.");
        }

        if let Value::BLOCK(block) = arguments[1].clone() {
            loop {
                let condition = if let Value::BLOCK(condition_block) = arguments[0].clone() {
                    if let Value::BOOL(condition) = self.interpret_block(condition_block) {
                        condition
                    } else {
                        self.error("While's condition must return a boolean (boolean must be the last expression's result).");
                    }
                } else {
                    self.error("While's condition must be a condition block returning a boolean (boolean must be the last expression's result).");
                };

                if !condition {
                    break;
                }

                self.begin_scope();
                if let Value::ERROR(error_message) = self.interpret_block(block.clone()) {
                    if error_message == "LoopExit".to_string() {
                        self.end_scope();
                        self.end_scope();
                        break;
                    }
                }
                self.end_scope();
            }
        }

        self.end_scope();

        Value::NULL
    }

    pub fn call_try(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            self.error("Try must have 2 arguments: a value and execution block.");
        }

        self.begin_scope();

        let result = if let Value::ERROR(error_message) = arguments[0].clone() {
            if let Value::BLOCK(block) = arguments[1].clone() {
                self.declare("error".to_string(), Value::STRING(error_message));
                let value = self.interpret_block(block.clone());
                self.end_scope();
                value
            } else {
                self.error("Try's second argument must be a block.");
            }
        } else {
            arguments[0].clone()
        };

        self.end_scope();

        result
    }

    pub fn call_for(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            self.error("For must have 2 arguments: a list or a string and execution block.");
        }

        self.begin_scope();

        if let Value::LIST(list) = arguments[0].clone() {
            if let Value::BLOCK(block) = arguments[1].clone() {
                for element in list {
                    self.begin_scope();
                    self.declare("element".to_string(), element);
                    if let Value::ERROR(error_message) = self.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            self.end_scope();
                            self.end_scope();
                            break;
                        }
                    }
                    self.end_scope();
                }
            } else {
                self.error("For's second argument must be a block.");
            }
        } else if let Value::STRING(string) = arguments[0].clone() {
            if let Value::BLOCK(block) = arguments[1].clone() {
                for element in string.chars() {
                    self.begin_scope();
                    self.declare("element".to_string(), Value::STRING(element.to_string()));
                    if let Value::ERROR(error_message) = self.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            self.end_scope();
                            self.end_scope();
                            break;
                        }
                    }
                    self.end_scope();
                }
            } else {
                self.error("For's second argument must be a block.");
            }
        } else if let Value::TABLE(table) = arguments[0].clone() {
            if let Value::BLOCK(block) = arguments[1].clone() {
                for (key, value) in table.iter() {
                    self.begin_scope();
                    self.declare("element".to_string(), Value::KEY_VALUE(key.to_string(), Box::new(value.clone())));
                    if let Value::ERROR(error_message) = self.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            self.end_scope();
                            self.end_scope();
                            break;
                        }
                    }
                    self.end_scope();
                }
            } else {
                self.error("For's second argument must be a block.");
            }
        } else {
            self.error("For's first argument must be a list.");
        };

        self.end_scope();

        Value::NULL
    }

    pub fn call_repeat(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 && arguments.len() != 1 {
            self.error("Repeat must have only 2 arguments: a number (optional) and execution block.");
        }

         self.begin_scope();

        if arguments.len() == 2 {
            let repetitions = if let Value::NUMBER(number) = arguments[0].clone() {
                if number < 1f64 {
                    self.error("Repeat's first argument must be a number greater than 0.");
                }
                number as i64
            } else {
                self.error("Repeat's first argument must be a number.");
            };

            if let Value::BLOCK(block) = arguments[1].clone() {
                for _ in 0..repetitions {
                    self.begin_scope();
                    if let Value::ERROR(error_message) = self.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            self.end_scope();
                            self.end_scope();
                            break;
                        }
                    }
                    self.end_scope();
                }
            }
        } else {
            loop {
                if let Value::BLOCK(block) = arguments[0].clone() {
                    self.begin_scope();
                    if let Value::ERROR(error_message) = self.interpret_block(block.clone()) {
                        if error_message == "LoopExit".to_string() {
                            self.end_scope();
                            self.end_scope();
                            break;
                        }
                    }
                    self.end_scope();
                }
            }
        }

        self.end_scope();

        Value::NULL
    }

    pub fn call_run(&mut self, arguments: Vec<Value>) -> Value {
        let mut last_result = Value::NULL;

        for argument in arguments {
            if let Value::BLOCK(block) = argument {
                last_result = self.interpret_block(block);
            } else {
                last_result = argument;
            }
        }

        last_result
    }

    pub fn call_map(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            self.error("Map function requires 2 arguments: a object and a block.");
        }

        let object = &arguments[0];
        let block = if let Value::BLOCK(block) = &arguments[1] {
            block
        } else {
            self.error("Map functions 2nd argument must be a block.");
        };

        match object {
            Value::LIST(list) => {
                let mut new_list: Vec<Value> = Vec::new();

                for element in list {
                    self.begin_scope();
                    self.declare("element".to_string(), element.clone());
                    new_list.push(self.interpret_block(block.clone()));
                    self.end_scope();
                }

                Value::LIST(new_list)
            },
            _ => {
                self.begin_scope();
                self.declare("element".to_string(), object.clone());
                let result = self.interpret_block(block.clone());
                self.end_scope();
                result
            }
        }
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

    pub fn call_read(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() == 0 {
            let mut line = String::new();
            if let Err(error_message) = io::stdin().read_line(&mut line) {
                self.error(&format!("Failed to read line: {}.", error_message));
            };
            if 1 < line.len() {
                line.remove(line.len() - 1);
            } else {
                line = "".to_string();
            }

            Value::STRING(line)
        } else {
            self.error("Read operation requires 0 arguments.");
        }
    }

    pub fn call_negate(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            self.error("Negation requires 1 boolean argument.");
        }

        if let Value::BOOL(boolean) = arguments[0] {
            Value::BOOL(!boolean)
        } else {
            self.error("Negation requires 1 boolean argument.");
        }
    }

    pub fn call_import(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            self.error("Import requires 1 string argument.");
        }

        if let Value::STRING(path) = arguments[0].clone() {
            let mut origin_path = self.path.clone();

            if origin_path.starts_with("./") {
                origin_path.remove(0);
                origin_path.remove(0);
                origin_path = std::env::current_dir().unwrap().to_str().unwrap().to_string() + "/" + &origin_path;
            }

            if !origin_path.ends_with("/") {
                let mut origin_path_split = origin_path.split("/").collect::<Vec<&str>>();
                origin_path_split.remove(origin_path_split.len() - 1);
                origin_path = origin_path_split.join("/");
                origin_path += "/"
            }

            let mut path = path;

            if path.starts_with("./") {
                path.remove(0);
                path.remove(0);
                path = origin_path.clone() + &path;
            }

            if !path.starts_with("/") {
                path = origin_path.clone() + &path;
            }

            if !path.ends_with(".hier") {
                path += ".hier";
            }

            let contents = (self.module_reader)(path.clone());

            let mut tokenizer = Tokenizer::new_with_name(contents, path.clone());

            if tokenizer.tokenize_module() {
                eprintln!("Failed to import file {}.", path);
                (self.exit_handler)()
            }

            let mut parser = Parser::new(tokenizer.tokens, self.module_reader, self.exit_handler);

            if parser.parse() {
                println!("Failed.");
                (self.exit_handler)();
            }

            let mut environment = Environment::new(false, path, self.module_reader, self.exit_handler);

            environment.code = parser.code;
            environment.interpret();

            Value::ENVIRONMENT(Box::new(environment))
        } else {
            self.error("Import requires 1 string argument.");
        }
    }

    pub fn call_number(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            self.error("Number conversion requires 1 argument.");
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
                self.error(&format!("Table operation's all arguments must be key-values, but {} was found.", argument.text_representation()));
            }
        }

        Value::TABLE(table)
    }

    pub fn call_string(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            self.error("String conversion requires 1 argument.");
        }

        Value::STRING(arguments[0].text_representation())
    }

    pub fn call_length(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            self.error("Length operation requires 1 argument that is an array (list or string).");
        }

        if let Value::LIST(list) = arguments[0].clone() {
            Value::NUMBER(list.len() as f64)
        } else if let Value::STRING(string) = arguments[0].clone() {
            Value::NUMBER(string.len() as f64)
        } else {
            self.error("Length operation requires 1 argument that is an array (list or string).");
        }
    }

    pub fn call_append(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 {
            self.error("Append operation requires 2 arguments: an array (list or string) and a value.");
        }

        if let Value::LIST(list) = arguments[0].clone() {
            let mut values = list;
            values.push(arguments[1].clone());
            Value::LIST(values)
        } else if let Value::STRING(string) = arguments[0].clone() {
            if let Value::STRING(new) = arguments[1].clone() {
                let mut e_string = string;
                e_string.push_str(&*new);
                Value::STRING(e_string)
            } else {
                self.error("Append expected a second string.");
            }
        } else {
            self.error("Append operation requires 2 arguments: an array (list or string) and a value.");
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
                self.error("Remove operation requires first argument to be an array (list or string).");
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
                    self.error("Remove operation requires first argument to be an array (list or string).");
                }
            } else {
                self.error("Remove operation requires second argument to be a number.");
            }
        } else {
            self.error("Remove operation requires 1 or 2 arguments: an array (list or string) and index (optional, if none, operate on last element).");
        }
    }

    pub fn call_replace(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 3 {
            self.error("Replace operation requires 3 arguments: an array (list or string), index and value.");
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
                    self.error("Replace operation requires third argument to be an string if array is a string.");
                }
            } else {
                self.error("Replace operation requires first argument to be an array (list or string).");
            }
        } else {
            self.error("Replace operation requires second argument to be a number.");
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
                    self.error("Insert operation requires second argument to be a string when array is a string.");
                }
            } else {
                self.error("Insert operation requires first argument to be an array (list or string).");
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
                        self.error("Insert operation requires second argument to be a string when array is a string.");
                    }
                } else {
                    self.error("Insert operation requires first argument to be an array (list or string).");
                }
            } else {
                self.error("Insert operation requires third argument to be a number.");
            }
        } else {
            self.error("Insert operation requires 2 or 3 arguments: an array (list or string), value and index (optional, if none, operate on last element).");
        }
    }

    pub fn call_break(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 0 {
            self.error("Break operation requires 0 arguments.");
        }

        Value::ERROR("LoopExit".to_string())
    }

    pub fn call_round(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            self.error("Round operation requires 1 number argument.");
        }

        if let Value::NUMBER(number) = arguments[0] {
            Value::NUMBER(number as i64 as f64)
        } else {
            self.error("Round operation requires a number argument.");
        }
    }

    pub fn call_error(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            self.error("Error operation requires 1 argument.");
        }

        Value::ERROR(arguments[0].clone().text_representation())
    }

    pub fn call_panic(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            self.error("Panic operation requires 1 argument.");
        }

        eprintln!("! Panic: {}", arguments[0].clone().text_representation());
        (self.exit_handler)();
    }

    pub fn call_eval(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 1 {
            self.error("Evaluate operation requires 1 string argument.");
        }

        if let Value::STRING(code) = arguments[0].clone() {
            let mut hier = Hier::new(self.path.clone(), self.module_reader, self.exit_handler);
            hier.run(code)
        } else {
            self.error("Evaluate operation requires a string argument.");
        }
    }

    pub fn call_get(&mut self, arguments: Vec<Value>) -> Value {
        if arguments.len() != 2 && arguments.len() != 1 {
            self.error("Get operation requires max 2 arguments: object and key (number or string, optional).");
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
                        self.error(&format!("Index {} is out of bounds ({} elements).", index, value.len()));
                    }

                    value[index as usize].clone()
                },
                Value::STRING(value) => {
                    if index < 0f64 || value.len() <= index as usize {
                        self.error(&format!("Index {} is out of bounds ({} elements).", index, value.len()));
                    }
                    Value::STRING(value.chars().nth(index as usize).clone().unwrap_or(' ').to_string())
                },
                _ => if index == 0f64 { arguments[0].clone() } else { Value::NULL },
            }
        } else {
            self.error("Get operation requires second arguments to be a number or string.");
        }
    }
}