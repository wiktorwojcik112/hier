use std::collections::HashMap;
use crate::expression::Expression;
use crate::types::Type;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum Value {
    LIST(Vec<Value>),
    STRING(String),
    NUMBER(f64),
    FUNCTION(Vec<String>, Box<Value>),
    BOOL(bool),
    NULL,
    BLOCK(Vec<Expression>),
    TYPE(Type),
    FUNCTION_ARGUMENTS(Vec<String>),
    KEY_VALUE(String, Box<Value>),
    TABLE(HashMap<String, Value>),
    ERROR(String)
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        if let Value::LIST(_) = self {
            return false;
        }

        if let Value::LIST(_) = other {
            return false;
        }

        if let Value::STRING(string1) = self {
            if let Value::STRING(string2) = other {
                return string1 == string2;
            } else {
                return false;
            }
        }

        if let Value::BOOL(bool1) = self {
            if let Value::BOOL(bool2) = other {
                return bool1 == bool2;
            } else {
                return false;
            }
        }

        self.text_representation() == other.text_representation()
    }
}

impl Eq for Value {

}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::STRING(_) => Type::STRING,
            Value::NUMBER(_) => Type::NUMBER,
            Value::BOOL(_) => Type::BOOL,
            Value::NULL => Type::NULL,
            Value::LIST(_) => Type::LIST,
            Value::FUNCTION(_, _) => Type::FUNCTION,
            Value::BLOCK(_) => Type::BLOCK,
            Value::TYPE(_) => Type::NULL,
            Value::FUNCTION_ARGUMENTS(_) => Type::FUNCTION_ARGUMENTS,
            Value::KEY_VALUE(_, _) => Type::KEY_VALUE,
            Value::TABLE(_) => Type::TABLE,
            Value::ERROR(_) => Type::ERROR
        }
    }

    pub fn text_representation(&self) -> String {
        match self {
            Value::STRING(value) => value.clone(),
            Value::NUMBER(value) => value.to_string(),
            Value::BOOL(value) => if *value { "true".to_string() } else { "false".to_string() },
            Value::NULL => "NULL".to_string(),
            Value::LIST(values) => self.text_representation_of_list(values),
            Value::FUNCTION(_, _) => "<FUNCTION>".to_string(),
            Value::BLOCK(_) => "<BLOCK>".to_string(),
            Value::TYPE(a_type) => a_type.text_representation(),
            Value::FUNCTION_ARGUMENTS(_) => "<FunctionArguments>".to_string(),
            Value::KEY_VALUE(key, value) => format!("{}({})", key, value.text_representation()),
            Value::TABLE(_) => "<TABLE>".to_string(),
            Value::ERROR(error) => error.to_string()
        }
    }

    fn text_representation_of_list(&self, values: &Vec<Value>) -> String {
        let mut final_string = String::new();

        for value in values {
            final_string += &(value.text_representation() + " ");
        }

        final_string
    }
}