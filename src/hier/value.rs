use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::hier::environment::Environment;
use crate::hier::expression::Expression;
use crate::hier::types::Type;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum Value {
    LIST(Vec<Value>),
    STRING(String),
    NUMBER(f64),
    FUNCTION(Vec<String>, Box<Value>),
    NATIVE_FUNCTION(fn(&mut Environment, Vec<Value>) -> Value, i64),
    BOOL(bool),
    NULL,
    BLOCK(Vec<Expression>),
    TYPE(Type),
    FUNCTION_ARGUMENTS(Vec<String>),
    KEY_VALUE(String, Box<Value>),
    TABLE(HashMap<String, Value>),
    ERROR(String),
    ENVIRONMENT(Box<Environment>)
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::LIST(list) => {
                let mut string = String::new();

                string.push('[');

                for value in list {
                    string.push_str(&format!("{:?}, ", value));
                }

                string.push(']');

                write!(f, "{}", string)
            },
            Value::STRING(string) => write!(f, "{}", string),
            Value::NUMBER(number) => write!(f, "{}", number),
            Value::FUNCTION(arguments, value) => write!(f, "Function {{ arguments: {:?}, value: {:?} }}", arguments, value),
            Value::NATIVE_FUNCTION(_, _) => write!(f, "Native function"),
            Value::BOOL(boolean) => write!(f, "{}", boolean),
            Value::NULL => write!(f, "null"),
            Value::BLOCK(expressions) => write!(f, "Block {{ expressions: {:?} }}", expressions),
            Value::TYPE(type_) => write!(f, "Type {{ type: {:?} }}", type_),
            Value::FUNCTION_ARGUMENTS(arguments) => write!(f, "Function arguments {{ arguments: {:?} }}", arguments),
            Value::KEY_VALUE(key, value) => write!(f, "Key value {{ key: {:?}, value: {:?} }}", key, value),
            Value::TABLE(table) => write!(f, "Table {{ table: {:?} }}", table),
            Value::ERROR(error) => write!(f, "Error {{ error: {:?} }}", error),
            Value::ENVIRONMENT(_) => write!(f, "Environment"),
        }
    }
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
            return if let Value::STRING(string2) = other {
                string1 == string2
            } else {
                false
            }
        }

        if let Value::BOOL(bool1) = self {
            if let Value::BOOL(bool2) = other {
                return bool1 == bool2;
            } else {
                return false;
            }
        }

        if let Value::ENVIRONMENT(_) = self {
            return false;
        }

        if let Value::ENVIRONMENT(_) = other {
            return false;
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
            Value::ERROR(_) => Type::ERROR,
            Value::NATIVE_FUNCTION(_, _) => Type::FUNCTION,
            Value::ENVIRONMENT(_) => Type::ENVIRONMENT
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
            Value::FUNCTION_ARGUMENTS(_) => "<FUNCTION_ARGUMENTS>".to_string(),
            Value::KEY_VALUE(key, value) => format!("{}({})", key, value.text_representation()),
            Value::TABLE(_) => "<TABLE>".to_string(),
            Value::ERROR(error) => error.to_string(),
            Value::NATIVE_FUNCTION(_, _) => "<FUNCTION>".to_string(),
            Value::ENVIRONMENT(_) => "<ENVIRONMENT>".to_string()
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