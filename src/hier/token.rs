use core::fmt;
use std::fmt::{Debug, Formatter};
use crate::hier::location::Location;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Token {
    LEFT_BRACKET(Location),
    RIGHT_BRACKET(Location),
    LEFT_CURLY(Location),
    RIGHT_CURLY(Location),
    LEFT_SQUARE(Location),
    RIGHT_SQUARE(Location),
    NUMBER(f64, Location),
    STRING(String, Location),
    IDENTIFIER(String, Location),
    DOT(Location),
    COLON(Location),
    EXCL_MARK(Location)

}

impl Token {
    pub fn get_location(&self) -> &Location {
        match self {
            Token::LEFT_BRACKET(location) => location,
            Token::RIGHT_BRACKET(location) => location,
            Token::LEFT_CURLY(location) => location,
            Token::RIGHT_CURLY(location) => location,
            Token::NUMBER(_, location) => location,
            Token::STRING(_, location) => location,
            Token::IDENTIFIER(_, location) => location,
            Token::DOT(location) => location,
            Token::LEFT_SQUARE(location) => location,
            Token::RIGHT_SQUARE(location) => location,
            Token::COLON(location) => location,
            Token::EXCL_MARK(location) => location
        }
    }

    pub fn get_symbol(&self) -> String {
        match self {
            Token::LEFT_BRACKET(_) => "(".to_string(),
            Token::RIGHT_BRACKET(_) => ")".to_string(),
            Token::LEFT_CURLY(_) => "{".to_string(),
            Token::RIGHT_CURLY(_) => "}".to_string(),
            Token::LEFT_SQUARE(_) => "[".to_string(),
            Token::RIGHT_SQUARE(_) => "]".to_string(),
            Token::NUMBER(number, _) => number.to_string(),
            Token::STRING(string, _) => "\"".to_string() + string + "\"",
            Token::IDENTIFIER(identifier, _) => identifier.to_string(),
            Token::DOT(_) => ".".to_string(),
            Token::COLON(_) => ":".to_string(),
            Token::EXCL_MARK(_) => "!".to_string()
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}