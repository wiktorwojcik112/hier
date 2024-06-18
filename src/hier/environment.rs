use std::collections::HashMap;
use crate::hier::expression::Expression;
use crate::hier::interpreter::warning;
use crate::hier::location::Location;
use crate::hier::{debugger, report};
use crate::hier::value::Value;


type Scope = u64;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct VariableId(pub Scope, pub String);

#[derive(Clone)]
pub struct Environment {
    pub scope: Scope,
    pub path: String,
    pub values: HashMap<VariableId, Value>,
    pub code: Expression,
    is_in_repl: bool,
    pub module_reader: fn(String) -> String,
    pub exit_handler: fn() -> !,
    pub current_interpreting_location: Location,
    pub current_interpreting_expression: Expression,
    pub is_debugging: bool,
    pub breakpoints: Vec<String>,
    pub is_a_step: bool
}

impl Environment {
    pub fn new(is_in_repl: bool, path: String, module_reader: fn(String) -> String, exit_handler: fn() -> !, is_debugging: bool, breakpoints: Vec<String>) -> Self {
        Self {
            scope: 0,
            values: HashMap::new(),
            code: Expression::LIST(vec![], Location::empty()),
            path,
            is_in_repl,
            module_reader,
            exit_handler,
            current_interpreting_location: Location::empty(),
            current_interpreting_expression: Expression::VALUE(Value::NULL),
            is_debugging,
            breakpoints,
            is_a_step: false
        }
    }

    pub fn error(&self, error: &str) -> ! {
        report(error, self.current_interpreting_location.clone());

        if self.is_in_repl {
            panic!("");
        } else {
            if self.is_debugging {
                let mut editable = self.clone();
                debugger::debug(&mut editable, &String::from("ERROR"));
            }

            (self.exit_handler)()
        }
    }

    pub fn new_with_code(code: Expression, is_in_repl: bool, module_reader: fn(String) -> String, exit_handler: fn() -> !, is_debugging: bool, breakpoints: Vec<String>) -> Self {
        Self {
            scope: 0,
            values: HashMap::new(),
            code,
            path: String::new(),
            is_in_repl,
            module_reader,
            exit_handler,
            current_interpreting_location: Location::empty(),
            current_interpreting_expression: Expression::VALUE(Value::NULL),
            is_debugging,
            breakpoints,
            is_a_step: false
        }
    }
    pub fn begin_scope(&mut self) {
        self.scope += 1;
    }

    pub fn end_scope(&mut self) {
        if self.scope == 0 {
            self.error("Ended scope that didn't exist.");
        } else {
            for (key, _) in self.values.clone() {
                if key.0 == self.scope {
                    self.values.remove(&key);
                }
            }

            self.scope -= 1;
        }
    }

    pub fn get(&self, key: String) -> Value {
        if key.contains("::") {
            // We can unwrap, because it is nil only if the delimiter is not present, but we can be sure, because we checked.
            let path = key.split_once("::").unwrap();

            if path.1.to_string().chars().collect::<Vec<char>>()[0] == '_' {
                return Value::NULL;
            }

            let environment = self.get(path.0.to_string());

            if let Value::ENVIRONMENT(target_environment) = environment {
                target_environment.get(path.1.to_string())
            } else if let Value::NULL = environment {
                return Value::NULL;
            } else {
                let target = path.0;
                self.error(&format!("{target} is not an environment."));
            }
        } else {
            if let Some(value) = self.values.get(&VariableId(self.scope, key.clone())) {
                value.clone()
            } else {
                if self.scope == 0 {
                    Value::NULL
                } else {
                    self.get_in_scope(key, self.scope - 1)
                }
            }
        }
    }

    fn get_in_scope(&self, key: String, scope: Scope) -> Value {
        if key.contains("::") {
            // We can unwrap, because it is nil only if the delimiter is not present, but we can be sure, because we checked.
            let path = key.split_once("::").unwrap();

            if path.1.to_string().chars().collect::<Vec<char>>()[0] == '_' {
                return Value::NULL;
            }

            let environment = self.values.get(&VariableId(scope, key.clone()));

            if let Some(value) = environment {
                if let Value::ENVIRONMENT(target_environment) = value {
                    target_environment.get(key)
                } else if let Value::NULL = value {
                    return Value::NULL;
                } else {
                    let target = path.0;
                    self.error(&format!("{target} is not an environment."));
                }
            } else {
                Value::NULL
            }
        } else {
            if let Some(value) = self.values.get(&VariableId(scope, key.clone())) {
                value.clone()
            } else {
                if scope == 0 {
                    Value::NULL
                } else {
                    self.get_in_scope(key, scope - 1)
                }
            }
        }
    }

    /// Declare a new variable in current scope and assign it some value
    pub fn declare(&mut self, key: String, value: Value) {
        if self.is_in_repl {
            self.values.insert(VariableId(self.scope, key), value);
        } else {
            if let Some(_) = self.values.get(&VariableId(self.scope, key.clone())) {
                self.error(&format!("Variable '{}' already exists in current scope.", key));
            } else {
                self.values.insert(VariableId(self.scope, key), value);
            }
        }
    }

    /// Assign a value to a variable and error when it already exists
    pub fn assign(&mut self, key: String, value: Value) {
        if self.values.contains_key(&VariableId(self.scope, key.clone())) {
            self.values.insert(VariableId(self.scope, key.clone()), value);
        } else {
            if self.scope == 0 {
                self.error(&format!("Variable {} doesn't exist.", key));
            } else {
                self.assign_in_scope(key, value, self.scope - 1)
            }
        }
    }

    fn assign_in_scope(&mut self, key: String, value: Value, scope: Scope) {
        if self.values.contains_key(&VariableId(scope, key.clone())) {
            self.values.insert(VariableId(scope, key.clone()), value);
        } else {
            if scope == 0 {
                self.error(&format!("Variable {} doesn't exist.", key));
            } else {
                self.assign_in_scope(key, value, scope - 1);
            }
        }
    }

    fn call_user_defined_function(&mut self, name: &String, arguments: Vec<Value>) -> Value {
        if let Value::FUNCTION(parameters, block) = self.get(name.clone().to_string()) {
            if arguments.len() != parameters.len() {
                self.error(&format!("Function {} expects {} arguments, but {} were provided.", name, parameters.len(), arguments.len()));
            }

            {
                self.begin_scope();

                for (i, argument) in arguments.iter().enumerate() {
                    self.declare(parameters[i].clone(), argument.clone());
                }

                if let Value::BLOCK(block) = (*block).clone() {
                    let value = self.interpret_block(block);
                    self.end_scope();
                    value
                } else {
                    Value::NULL
                }
            }
        } else if let Value::NATIVE_FUNCTION(function, arity) = self.get(name.clone().to_string()) {
            if arity != -1 && arguments.len() != arity as usize {
                self.error(&format!("Function {} expects {} arguments, but {} were provided.", name, arity, arguments.len()));
            }

            function(self, arguments)
        } else {
            warning(&format!("Function {} doesn't exist or is not a function.", name));
            Value::NULL
        }
    }

    pub fn call_function(&mut self, name: &String, arguments: Vec<Value>) -> Value {
        if (self.breakpoints.contains(name) || self.is_a_step) && self.is_debugging {
            debugger::debug(self, name);
        }

        if name.contains("::") {
            let path = name.split_once("::").unwrap();

            if path.1.to_string().chars().collect::<Vec<char>>()[0] == '_' {
                return Value::NULL;
            }

            let environment = self.get(path.0.to_string());

            if let Value::ENVIRONMENT(target_environment) = environment {
                let mut environment = target_environment.clone();

                let result = environment.call_function(&path.1.to_string(), arguments);

                self.assign(path.0.to_string(), Value::ENVIRONMENT(environment));

                result
            } else if let Value::NULL = environment {
                return Value::NULL;
            } else {
                let target = path.0;
                self.error(&format!("{target} is not an environment."));
            }
        } else {
            match &name as &str {
                "get" => self.call_get(arguments),
                "import" => self.call_import(arguments),
                "&" | "list" => self.call_list(arguments),
                "+" => self.call_addition(arguments),
                "-" => self.call_subtraction(arguments),
                "*" => self.call_multiplication(arguments),
                "/" => self.call_division(arguments),
                "!" => self.call_negate(arguments),
                "&&" | "||" => self.call_logical(name, arguments),
                "==" | "!=" | "<=" | ">=" | "<" | ">" => self.call_comparison(name, arguments),
                "??" => self.call_null_coalescing(arguments),
                "append" => self.call_append(arguments),
                "brpoint" => self.call_brpoint(arguments),
                "%" => self.call_modulo(arguments),
                "is" => self.call_is(arguments),
                "print" => self.call_print(arguments),
                "println" => self.call_println(arguments),
                "eval" => self.call_eval(arguments),
                "break" => self.call_break(arguments),
                "error" => self.call_error(arguments),
                "panic" => self.call_panic(arguments),
                "read" => self.call_read(arguments),
                "insert" => self.call_insert(arguments),
                "round" => self.call_round(arguments),
                "map" => self.call_map(arguments),
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
                        // Process declaration
                        if name == "@" {
                            self.error("Name can't be empty (can't be only @).")
                        }

                        let mut name = name.clone();
                        name.remove(0);

                        if 2 < arguments.len() {
                            self.declare(name, Value::LIST(arguments.clone()));
                            Value::LIST(arguments)
                        } else if arguments.len() == 2 {
                            if let Value::FUNCTION_ARGUMENTS(parameters) = arguments[0].clone() {
                                if let Value::BLOCK(block) = arguments[1].clone() {
                                    self.declare(name, Value::FUNCTION(parameters.clone(), Box::new(Value::BLOCK(block.clone()))));
                                    Value::FUNCTION(parameters, Box::new(Value::BLOCK(block)))
                                } else {
                                    self.error("Function definition's second argument must be a block.");
                                }
                            } else {
                                self.error("Function definition's first argument must be function arguments.");
                            }
                        } else if arguments.len() == 1 {
                            self.declare(name, arguments[0].clone());
                            arguments[0].clone()
                        } else {
                            self.error("Variable set operation must have 1 or more arguments.");
                        }
                    } else if name.chars().nth(0).unwrap_or(' ') == '=' {
                        // Process assignment
                        if name == "=" {
                            self.error("Name can't be empty (can't be only =).")
                        }

                        let mut name = name.clone();
                        name.remove(0);

                        if 2 < arguments.len() {
                            self.assign(name, Value::LIST(arguments.clone()));
                            Value::LIST(arguments)
                        } else if arguments.len() == 2 {
                            if let Value::FUNCTION_ARGUMENTS(parameters) = arguments[0].clone() {
                                if let Value::BLOCK(block) = arguments[1].clone() {
                                    self.assign(name, Value::FUNCTION(parameters.clone(), Box::new(Value::BLOCK(block.clone()))));
                                    Value::FUNCTION(parameters, Box::new(Value::BLOCK(block)))
                                } else {
                                    self.error("Function definition's second argument must be a block.");
                                }
                            } else {
                                self.error("Function definition's first argument must be function arguments.");
                            }
                        } else if arguments.len() == 1 {
                            self.assign(name, arguments[0].clone());
                            arguments[0].clone()
                        } else {
                            self.error("Variable set operation must have 1 or more arguments.");
                        }
                    } else {
                        self.call_user_defined_function(name, arguments)
                    }
                }
            }
        }
    }
}