use crate::hier::debugger;
use crate::hier::environment::{Environment, VariableId};
use crate::hier::parser::Parser;
use crate::hier::tokenizer::Tokenizer;
use crate::hier::value::Value;

pub struct Hier {
    environment: Environment,
    module_reader: fn(String) -> String,
    exit_handler: fn() -> !,
    pub debug: bool
}

impl Hier {
    pub fn new(path: String, module_reader: fn(String) -> String, exit_handler: fn() -> !, debug: bool) -> Self {
        Self {
            environment: Environment::new(false, path, module_reader, exit_handler, debug, vec![]),
            module_reader,
            exit_handler,
            debug
        }
    }

    pub fn run(&mut self, code: String) -> Value {
        let mut code = code;

        if !code.starts_with('(') {
            code.insert(0, '(');
            code.push(')');
        }

        let mut tokenizer = Tokenizer::new(code);

        if tokenizer.tokenize_module() {
            println!("Failed.");
            (self.exit_handler)();
        }

        let mut parser = Parser::new(tokenizer.tokens, self.module_reader, self.exit_handler);

        if parser.parse() {
            println!("Failed.");
            (self.exit_handler)();
        }

        self.environment.code = parser.code;

        if self.debug {
            debugger::debug(&mut self.environment, &String::new());
        }

        self.environment.interpret()
    }

    pub fn add_function(&mut self, name: String, arguments_count: i64, function: fn(&mut Environment, Vec<Value>) -> Value) {
        if arguments_count < -1 {
            panic!("Invalid argument count for function {}. Must be either -1 (infinite) or 0 and higher.", name);
        }

        self.environment.values.insert(VariableId(0, name), Value::NATIVE_FUNCTION(function, arguments_count));
    }

    pub fn add_variable(&mut self, name: String, value: Value) {
        self.environment.values.insert(VariableId(0, name), value);
    }
}