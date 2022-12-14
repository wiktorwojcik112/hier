use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use crate::expression::Expression;
use crate::Interpreter;
use crate::interpreter::{error, warning};
use crate::value::Value;

#[derive(Clone)]
pub struct Environment {
    parent: Option<Box<Environment>>,
    pub values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        let mut values: HashMap<String, Value> = HashMap::new();

        values.insert("cwd".to_string(), match current_dir() {
            Ok(path) => Value::STRING(path.to_str().unwrap().to_string()),
            Err(_) => Value::NULL,
        });

        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        values.insert("args".to_string(), Value::LIST(args.iter().map(|arg| Value::STRING(arg.to_string())).collect()));

        Self {
            parent: None,
            values
        }
    }

    pub fn child(&self) -> Self {
        Self {
            parent: Some(Box::new(self.clone())),
            values: HashMap::new()
        }
    }

    pub fn restore(&mut self, environment: Environment) {
        if let Some(environment) = environment.parent.clone() {
            self.values = (*environment).values;
        }
    }

    pub fn get(&self, key: String) -> Value {
        if let None = self.parent {
            (*self.values.get(&key).unwrap_or(&Value::NULL)).clone()
        } else {
            if let Some(value) = self.values.get(&key) {
                (*value).clone()
            } else {
                (*self.parent.clone().unwrap()).get(key).clone()
            }
        }
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    fn call_user_defined_function(&mut self, name: &String, arguments: Vec<Value>) -> Value {
        if let Value::FUNCTION(parameters, block) = self.get(name.clone().to_string()) {
            if arguments.len() != parameters.len() {
                error(&format!("Function {} expects {} arguments, but {} were provided.", name, parameters.len(), arguments.len()));
            }

            let mut environment = self.child();

            for (i, argument) in arguments.iter().enumerate() {
                environment.set(parameters[i].clone(), argument.clone());
            }

            let mut interpreter = Interpreter::new(Expression::NUMBER(0f64), &mut environment);

            if let Value::BLOCK(block) = (*block).clone() {
                interpreter.interpret_block(block)
            } else {
                Value::NULL
            }
        } else {
            warning(&format!("Function {} doesn't exist or is not a function.", name));
            Value::NULL
        }
    }

    pub fn call_function(&mut self, name: &String, arguments: Vec<Value>) -> Value {
        match &name as &str {
            "get" => self.call_get(arguments),
            "&" | "list" => self.call_list(arguments),
            "+" => self.call_addition(arguments),
            "-" => self.call_subtraction(arguments),
            "*" => self.call_multiplication(arguments),
            "/" => self.call_division(arguments),
            "!" => self.call_negate(arguments),
            "&&" | "||" => self.call_logical(name, arguments),
            "is" | "??" | "==" | "!=" | "<=" | ">=" | "<" | ">" | "%" => self.call_binary(name, arguments),
            "print" => self.call_print(arguments),
            "println" => self.call_println(arguments),
            "time" => self.call_time(arguments),
            "random" => self.call_random(arguments),
            "eval" => self.call_eval(arguments),
            "break" => self.call_break(arguments),
            "error" => self.call_error(arguments),
            "panic" => self.call_panic(arguments),
            "cmd" => self.call_cmd(arguments),
            "read" => self.call_read(arguments),
            "insert" => self.call_insert(arguments),
            "write" => self.call_write(arguments),
            "round" => self.call_round(arguments),
            "remove" => self.call_remove(arguments),
            "replace" => self.call_replace(arguments),
            "length" => self.call_length(arguments),
            "string" => self.call_string(arguments),
            "number" => self.call_number(arguments),
            "if" => self.call_if(arguments),
            "while" => self.call_while(arguments),
            "table" | "#" => self.call_table(arguments),
            "repeat" => self.call_repeat(arguments),
            "for" => self.call_for(arguments),
            "run" => self.call_run(arguments),
            "try" => self.call_try(arguments),
            _ => {
                if name.chars().nth(0).unwrap_or(' ') == '@' {
                    if name == "@" {
                        error("Name can't be empty (can't be only @).")
                    }

                    let mut name = name.clone();
                    name.remove(0);

                    if 2 < arguments.len() {
                        self.call_set(name, Value::LIST(arguments))
                    } else if arguments.len() == 2 {
                        if let Value::FUNCTION_ARGUMENTS(parameters) = arguments[0].clone() {
                            if let Value::BLOCK(block) = arguments[1].clone() {
                                self.call_set(name, Value::FUNCTION(parameters, Box::new(Value::BLOCK(block))))
                            } else {
                                error("Function definition's second argument must be a block.");
                            }
                        } else {
                            error("Function definition's first argument must be function arguments.");
                        }
                    } else if arguments.len() == 1 {
                        self.call_set(name, arguments[0].clone())
                    } else {
                        error("Variable set operation must have 1 or more arguments.");
                    }
                } else {
                    self.call_user_defined_function(name, arguments)
                }
            }
        }
    }
}