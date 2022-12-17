use crate::environment::Environment;
use crate::expression::Expression;
use crate::types::Type;
use crate::value::Value;

pub fn warning(error: &str) {
    eprintln!("?: {}", error);
}

impl Environment {
    pub fn interpret(&mut self) -> Value {
        if let Expression::BLOCK(block) = self.code.clone() {
            self.interpret_block(block)
        } else {
            self.visit(self.code.clone())
        }
    }

    pub fn interpret_block(&mut self, block: Vec<Expression>) -> Value {
        let mut last_result = Value::NULL;

        for expression in block {
            last_result = self.visit(expression);

            if let Value::ERROR(error_message) = last_result.clone() {
                if error_message == "LoopExit".to_string() {
                    break;
                }
            }
        }

        last_result
    }

    pub fn visit(&mut self, value: Expression) -> Value {
        match value {
            Expression::STRING(_) => self.visit_string(value),
            Expression::VALUE(_) => self.visit_value(value),
            Expression::NUMBER(_) => self.visit_number(value),
            Expression::IDENTIFIER(_) => self.visit_identifier(value),
            Expression::LIST(_) => self.visit_list(value),
            Expression::BLOCK(_) => self.visit_block(value),
            Expression::KEY_VALUE(_, _) => self.visit_key_value(value),
            Expression::PROPERTY(_, _) => self.visit_property(value)
        }
    }

    pub fn visit_property(&mut self, property: Expression) -> Value {
        if let Expression::PROPERTY(expression, identifier) = property {
            let argument = self.visit(*(expression.clone()));
            self.call_function(&("get".to_string()), vec![argument, Value::STRING(identifier)])
        } else {
            Value::NULL
        }
    }

    pub fn visit_list(&mut self, list: Expression) -> Value {
        if let Expression::LIST(list) = list {
            if list.len() == 0 {
                Value::NULL
            } else {
                if let Expression::IDENTIFIER(name) = &list[0] {
                    if name == "|" {
                        let mut expressions = list.clone();
                        expressions.remove(0);

                        let mut arguments: Vec<String> = Vec::new();

                        for expression in expressions {
                            if let Expression::IDENTIFIER(argument) = expression {
                                arguments.push(argument);
                            } else {
                                self.error("Function arguments must be identifiers.");
                            }
                        }

                        Value::FUNCTION_ARGUMENTS(arguments)
                    } else {
                        let mut expressions = list.clone();
                        expressions.remove(0);

                        let mut values: Vec<Value> = Vec::new();

                        for expression in expressions {
                            values.push(self.visit(expression));
                        }

                        self.call_function(&name, values)
                    }
                } else if let Expression::PROPERTY(expression, identifier) = &list[0] {
                    let mut expressions = list.clone();
                    expressions.remove(0);
                    expressions.insert(0, (**expression).clone());

                    let mut values: Vec<Value> = Vec::new();

                    for expression in expressions {
                        values.push(self.visit(expression));
                    }

                    self.call_function(identifier, values)
                } else {
                    if let Value::FUNCTION_ARGUMENTS(arguments) = self.visit(list[0].clone()) {
                        if list.len() != 2 {
                            self.error("Anonymous function's must have 2 arguments: function arguments and a block");
                        }

                        if let Expression::BLOCK(block) = list[1].clone() {
                            return Value::FUNCTION(arguments, Box::new(Value::BLOCK(block)));
                        } else {
                            self.error("Anonymous function's second argument must be a block.");
                        }
                    }

                    let expressions = list.clone();
                    let mut values: Vec<Value> = Vec::new();

                    for expression in expressions {
                        values.push(self.visit(expression));
                    }

                    Value::LIST(values)
                }
            }
        } else {
            Value::NULL
        }
    }

    pub fn visit_identifier(&mut self, value: Expression) -> Value {
        if let Expression::IDENTIFIER(value) = value {
            match &value as &str {
                "true" => Value::BOOL(true),
                "false" => Value::BOOL(false),
                "null" => Value::NULL,
                _ => {
                    if let Some(a_type) = Type::get_for_name(&value) {
                        Value::TYPE(a_type)
                    } else {
                        self.get(value)
                    }
                }
            }
        } else {
            Value::NULL
        }
    }

    pub fn visit_value(&mut self, value: Expression) -> Value {
        if let Expression::VALUE(value) = value {
            value
        } else {
            Value::NULL
        }
    }

    pub fn visit_key_value(&mut self, value: Expression) -> Value {
        if let Expression::KEY_VALUE(identifier, expression) = value {
            Value::KEY_VALUE(identifier, Box::new(self.visit(*expression)))
        } else {
            Value::NULL
        }
    }

    pub fn visit_string(&mut self, value: Expression) -> Value {
        if let Expression::STRING(value) = value {
            Value::STRING(value)
        } else {
            Value::NULL
        }
    }

    pub fn visit_number(&mut self, value: Expression) -> Value {
        if let Expression::NUMBER(value) = value {
            Value::NUMBER(value)
        } else {
            Value::NULL
        }
    }

    pub fn visit_block(&mut self, value: Expression) -> Value {
        if let Expression::BLOCK(value) = value {
            Value::BLOCK(value)
        } else {
            Value::NULL
        }
    }
}