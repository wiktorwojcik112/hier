use crate::hier::interpolated_string::InterpolatedString;
use crate::hier::location::Location;
use crate::hier::value::Value;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Expression {
    STRING(InterpolatedString, Location),
    NUMBER(f64, Location),
    IDENTIFIER(String, Location),
    LIST(Vec<Expression>, Location),
    BLOCK(Vec<Expression>, Location),
    PROPERTY(Box<Expression>, String, Location),
    VALUE(Value),
    KEY_VALUE(String, Box<Expression>, Location)
}

impl Expression {
    pub fn get_location(&self) -> Location {
        match self {
            Expression::STRING(_, location) => location.clone(),
            Expression::NUMBER(_, location) => location.clone(),
            Expression::IDENTIFIER(_, location) => location.clone(),
            Expression::LIST(_, location) => location.clone(),
            Expression::BLOCK(_, location) => location.clone(),
            Expression::PROPERTY(_, _, location) => location.clone(),
            Expression::VALUE(_) => Location::empty(),
            Expression::KEY_VALUE(_, _, location) => location.clone()
        }
    }
}