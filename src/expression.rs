use crate::value::Value;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Expression {
    STRING(String),
    NUMBER(f64),
    IDENTIFIER(String),
    LIST(Vec<Expression>),
    BLOCK(Vec<Expression>),
    PROPERTY(Box<Expression>, String),
    VALUE(Value),
    KEY_VALUE(String, Box<Expression>)
}