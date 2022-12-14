use std::fs;
use std::process::exit;
use crate::location::Location;
use crate::report;
use crate::token::Token;

pub struct Tokenizer {
    code: String,
    current_index: usize,
    current_line: i64,
    current_offset: i64,
    pub tokens: Vec<Token>,
    had_error: bool,
    modules: Vec<String>,
    pub(crate) module_name: String
}

impl Tokenizer {
    pub fn new(code: String) -> Self {
        Self {
            code,
            current_index: 0,
            current_line: 1,
            current_offset: 0,
            tokens: vec![],
            had_error: false,
            modules: Vec::new(),
            module_name: String::new()
        }
    }

    /// Returns bool if there was a error.
    pub fn tokenize(&mut self) -> bool {
        self.tokens.push(Token::LEFT_CURLY(self.make_location()));

        self.tokenize_module();

        self.tokens.push(Token::RIGHT_CURLY(self.make_location()));

        self.had_error
    }

    pub fn tokenize_module(&mut self) -> bool {
        while self.current_index < self.code.len() {
            let current_char = self.peek();

            if self.peek() == '#' && self.peek_next() != ' ' {
                self.process();
            } else if current_char == '\\' && self.peek_next() == '*' {
                self.consume();
                self.consume();
                self.comment();
            } else if current_char == '\n' {
                self.consume();

                if self.peek() == '#' && self.peek_next() != ' ' {
                    self.process();
                }

                self.current_line += 1;
                self.current_offset = 0;
                continue;
            } else if current_char == ' ' || current_char == '\t' {
                self.consume();
            } else if current_char == '.' {
                self.tokens.push(Token::DOT(self.make_location()));
                self.consume();
            } else if current_char == ':' {
                self.tokens.push(Token::COLON(self.make_location()));
                self.consume();
            } else if current_char == '(' {
                self.tokens.push(Token::LEFT_BRACKET(self.make_location()));
                self.consume();
            } else if current_char == ')' {
                self.tokens.push(Token::RIGHT_BRACKET(self.make_location()));
                self.consume();
            } else if current_char == '[' {
                self.tokens.push(Token::LEFT_SQUARE(self.make_location()));
                self.consume();
            } else if current_char == ']' {
                self.tokens.push(Token::RIGHT_SQUARE(self.make_location()));
                self.consume();
            } else if current_char == '{' {
                self.tokens.push(Token::LEFT_CURLY(self.make_location()));
                self.consume();
            } else if current_char == '}' {
                self.tokens.push(Token::RIGHT_CURLY(self.make_location()));
                self.consume();
            } else if current_char == '"' {
                self.string();
            } else {
                if Tokenizer::is_a_digit(current_char) {
                    self.number();
                } else {
                    self.identifier();
                }
            }
        }

        self.had_error
    }

    fn process(&mut self) {
        self.consume();

        if self.peek() == '<' {
            self.consume();
            let mut path = String::new();

            while self.current_index < self.code.len() && self.peek() != '>' {
                path += &(self.consume().to_string());
            }

            self.consume();

            if self.modules.contains(&path) {
                return;
            }

            let contents = fs::read_to_string(path.clone())
                .expect(&format!("Unable to read the file: {}", path));

            let mut tokenizer = Tokenizer::new(contents);

            tokenizer.modules = self.modules.clone();
            tokenizer.module_name = path.clone();

            if tokenizer.tokenize_module() {
                eprintln!("Failed to include file {}.", path);
                self.had_error = true;
                exit(1);
            }

            for token in tokenizer.tokens.clone() {
                self.tokens.push(token);
            }

            self.modules.push(tokenizer.module_name);
        } else {
            let mut module_name = String::new();

            while self.current_index < self.code.len() && self.peek() != '\n' {
                module_name += &(self.consume().to_string());
            }

            self.module_name = module_name;
            return;
        }
    }

    fn comment(&mut self) {
        while self.current_index < self.code.len() && !(self.peek() == '*' && self.peek_next() == '\\') {
            self.consume();
        }

        self.consume();
        self.consume();
    }

    fn identifier(&mut self) {
        let mut identifier = String::new();

        while self.current_index < self.code.len() && self.peek() != ' ' && self.peek() != ':' && self.peek() != '(' && self.peek() != ')' && self.peek() != '.' && self.peek() != '\n' && self.peek() != ']' && self.peek() != '[' {
            identifier.push(self.consume());
        }

        self.tokens.push(Token::IDENTIFIER(identifier, self.make_location()));
    }

    fn number(&mut self) {
        let mut number_string = String::new();

        let mut had_error = false;
        let mut is_first_character = true;


        while self.current_index < self.code.len() && self.peek() != ')' && self.peek() != ' ' && self.peek() != '\n' && self.peek() != ']' {
            if had_error {
                continue;
            }

            let current_char = self.consume();
            if Tokenizer::is_a_digit(current_char) {
                if !is_first_character && current_char == '-' {
                    report("- sign can be only present at the beginning of the number.", self.make_location());
                    had_error = true;
                }

                if is_first_character && current_char == '.' {
                    report(". must not be present at the beginning of the number.", self.make_location());
                    had_error = true;
                }

                number_string.push(current_char);
            } else {
                report(&format!("Character {} is disallowed in numbers. Only . - 0 1 2 3 4 5 6 7 8 9 characters are allowed.", current_char), self.make_location());
                had_error = true;
            }

            is_first_character = false;
        }

        if number_string == "-" {
            self.tokens.push(Token::IDENTIFIER("-".to_string(), self.make_location()));
            return;
        }

        if !had_error {
            let number = number_string.parse::<f64>();

            match number {
                Ok(number) => self.tokens.push(Token::NUMBER(number, self.make_location())),
                Err(_) => {
                    report(&format!("Number {} must have -?[0123456789]+(.[0123456789]+)? format.", number_string), self.make_location());
                    self.had_error = true;
                }
            }
        } else {
            self.had_error = true;
        }
    }

    fn string(&mut self) {
        self.consume();

        let mut string = String::new();

        while self.current_index < self.code.len() && self.peek() != '"' {
            let char = self.consume();

            if char == '\n' {
                self.current_line += 1;
                self.current_offset = 0;
            }

            string.push(char);
        }

        if self.consume() != '"' {
            report("Unterminated string.", self.make_location());
            self.had_error = true;
        }

        self.tokens.push(Token::STRING(string, self.make_location()));
    }

    fn make_location(&self) -> Location {
        Location::new(self.module_name.clone(), self.current_line, self.current_offset + 1)
    }

    fn is_a_digit(digit: char) -> bool {
        "-0123456789.".contains(digit)
    }

    fn peek(&self) -> char {
        self.code.chars().nth(self.current_index).unwrap_or(' ')
    }

    fn peek_next(&self) -> char {
        self.code.chars().nth(self.current_index + 1).unwrap_or(' ')
    }

    fn consume(&mut self) -> char {
        let char = self.code.chars().nth(self.current_index).unwrap_or(' ');
        self.current_index += 1;
        self.current_offset += 1;
        char
    }
}