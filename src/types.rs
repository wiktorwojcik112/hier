#[allow(non_camel_case_types)]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Type {
    LIST,
    STRING,
    NUMBER,
    FUNCTION,
    BOOL,
    NULL,
    BLOCK,
    TYPE,
    FUNCTION_ARGUMENTS,
    KEY_VALUE,
    TABLE,
    ERROR
}

impl Type {
    pub fn text_representation(&self) -> String {
        match self {
            Type::STRING => "String".to_string(),
            Type::NUMBER => "Number".to_string(),
            Type::BOOL => "Bool".to_string(),
            Type::NULL => "Null".to_string(),
            Type::LIST => "List".to_string(),
            Type::FUNCTION => "Function".to_string(),
            Type::BLOCK => "Block".to_string(),
            Type::TYPE => "Type".to_string(),
            Type::FUNCTION_ARGUMENTS => "FunctionArgs".to_string(),
            Type::KEY_VALUE => "KeyValue".to_string(),
            Type::TABLE => "Table".to_string(),
            Type::ERROR => "Error".to_string()
        }
    }

    pub fn get_for_name(name: &String) -> Option<Type> {
        match name as &str {
            "String" => Some(Type::STRING),
            "Number" => Some(Type::NUMBER),
            "Bool" => Some(Type::BOOL),
            "Null" => Some(Type::NULL),
            "List" => Some(Type::LIST),
            "Function" => Some(Type::FUNCTION),
            "Block" => Some(Type::BLOCK),
            "Type" => Some(Type::TYPE),
            "FunctionArgs" => Some(Type::FUNCTION_ARGUMENTS),
            "KeyValue" => Some(Type::KEY_VALUE),
            "Table" => Some(Type::TABLE),
            "Error" => Some(Type::ERROR),
            _ => None,
        }
    }
}