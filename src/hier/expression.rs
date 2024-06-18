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

    pub fn get_representation(&self) -> String {
        match self {
            Expression::STRING(value, _) => "\"".to_string() + &*(value.raw.clone()) + "\"",
            Expression::NUMBER(value, _) => value.to_string().clone(),
            Expression::IDENTIFIER(value, _) => value.clone(),
            Expression::PROPERTY(key, property, _) => key.get_representation() + "." + &*(property.clone()),
            Expression::VALUE(value) => value.text_representation().clone(),
            Expression::KEY_VALUE(key, value, _) => key.clone() + ":" + &*(value.get_representation()),
            Expression::LIST(expressions, _) => {
                let mut result = String::from("(");
                let mut i = 0;
                for expression in expressions {
                    result.push_str(&*(expression.get_representation()));

                    if i < expressions.len() - 1 {
                        result += " ";
                        i += 1;
                    }
                }

                result.push_str(")");
                result
            },
            Expression::BLOCK(expressions, _) => {
                let mut result = String::from("{ ");

                for expression in expressions {
                    result.push_str(&*(expression.get_representation()));
                    result += " ";
                }

                result.push_str("}");
                result
            }
        }
    }
}