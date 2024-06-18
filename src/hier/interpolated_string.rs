use crate::hier::expression::Expression;
use crate::hier::{Location, report};
use crate::hier::environment::Environment;
use crate::hier::parser::Parser;
use crate::hier::tokenizer::Tokenizer;

#[derive(Debug, Clone)]
enum InterpolatedStringPart {
    RAW(String),
    EXPRESSION(Expression)
}

#[derive(Debug, Clone)]
pub struct InterpolatedString {
    parts: Vec<InterpolatedStringPart>,
    pub raw: String,
    current_index: usize,
    pub module_reader: fn(String) -> String,
    pub exit_handler: fn() -> !

}

impl InterpolatedString {
    pub fn construct(string: String, location: Location, module_reader: fn(String) -> String, exit_handler: fn() -> !) -> InterpolatedString {
        let mut string = Self::new(string, module_reader, exit_handler);

        string.parse(location);

        string
    }

    pub fn new(string: String, module_reader: fn(String) -> String, exit_handler: fn() -> !) -> Self {
        Self {
            parts: vec![],
            raw: string,
            current_index: 0,
            module_reader,
            exit_handler
        }
    }

    pub fn parse(&mut self, location: Location) {
        let mut will_interpolate = false;
        let mut raw_part = String::new();

        while self.current_index < self.raw.len() {
            let current_char = self.peek();

            if current_char == '\\' {
                will_interpolate = true;
                self.consume();
            } else if will_interpolate && current_char == '(' {
                self.parts.push(InterpolatedStringPart::RAW(raw_part));
                raw_part = String::new();

                let mut tokenizer = Tokenizer::new(self.raw[self.current_index..].to_string());

                let offset = tokenizer.tokenize_interpolation();
                self.current_index += offset;

                let mut parser = Parser::new(tokenizer.tokens, self.module_reader, self.exit_handler);

                parser.parse();

                self.parts.push(InterpolatedStringPart::EXPRESSION(parser.code))
            } else if will_interpolate {
                match current_char {
                    'n' => raw_part.push('\n'),
                    't' => raw_part.push('\t'),
                    '0' => raw_part.push('\0'),
                    _ => report(&("Invalid escape sequence: \\".to_string() + &(current_char.to_string())), location.clone())
                }

                self.consume();
            } else {
                raw_part.push(current_char);
                self.consume();
            }
        }

        if !raw_part.is_empty() {
            self.parts.push(InterpolatedStringPart::RAW(raw_part));
        }
    }

    fn peek(&self) -> char {
        self.raw.chars().nth(self.current_index).unwrap_or(' ')
    }

    fn consume(&mut self) -> char {
        let char = self.raw.chars().nth(self.current_index).unwrap_or(' ');
        self.current_index += 1;
        char
    }

    pub fn resolve(&self, environment: &mut Environment) -> String {
        let mut resolved = String::new();

        for part in self.parts.clone() {
            match part {
                InterpolatedStringPart::RAW(raw) => resolved.push_str(&raw),
                InterpolatedStringPart::EXPRESSION(expression) => resolved.push_str(&environment.interpret_block(vec![expression]).text_representation())
            }
        }

        resolved
    }
}