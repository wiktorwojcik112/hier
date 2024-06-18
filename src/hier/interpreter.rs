use crate::hier::environment::Environment;
use crate::hier::expression::Expression;
use crate::hier::types::Type;
use crate::hier::value::Value;

pub fn warning(error: &str) {
    eprintln!("?: {}", error);
}

impl Environment {
    pub fn interpret(&mut self) -> Value {
        if let Expression::BLOCK(block, _) = self.code.clone() {
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
        self.current_interpreting_location = value.get_location().clone();

        match value {
            Expression::STRING(_, _) => self.visit_string(value),
            Expression::VALUE(_) => self.visit_value(value),
            Expression::NUMBER(_, _) => self.visit_number(value),
            Expression::IDENTIFIER(_, _) => self.visit_identifier(value),
            Expression::LIST(_, _) => self.visit_list(value),
            Expression::BLOCK(_, _) => self.visit_block(value),
            Expression::KEY_VALUE(_, _, _) => self.visit_key_value(value),
            Expression::PROPERTY(_, _, _) => self.visit_property(value)
        }
    }

    pub fn visit_property(&mut self, property: Expression) -> Value {
        self.current_interpreting_location = property.get_location().clone();

        if let Expression::PROPERTY(expression, identifier, _) = property {
            let argument = self.visit(*(expression.clone()));
            self.call_function(&("get".to_string()), vec![argument, Value::STRING(identifier)])
        } else {
            Value::NULL
        }
    }

    pub fn visit_list(&mut self, list: Expression) -> Value {
        let main = list.clone();
        self.current_interpreting_location = list.get_location().clone();

        if let Expression::LIST(list, _) = list {
            if list.len() == 0 {
                Value::LIST(vec![])
            } else {
                if let Expression::IDENTIFIER(name, _) = &list[0] {
                    if name == "|" {
                        let mut expressions = list.clone();
                        expressions.remove(0);

                        let mut arguments: Vec<String> = Vec::new();

                        for expression in expressions {
                            if let Expression::IDENTIFIER(argument, _) = expression {
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

                        self.current_interpreting_expression = main;
                        self.call_function(&name, values)
                    }
                } else if let Expression::PROPERTY(expression, identifier, _) = &list[0] {
                    let mut expressions = list.clone();
                    expressions.remove(0);
                    expressions.insert(0, (**expression).clone());

                    let mut values: Vec<Value> = Vec::new();

                    for expression in expressions {
                        values.push(self.visit(expression));
                    }

                    self.current_interpreting_expression = main;
                    self.call_function(identifier, values)
                } else {
                    if let Value::FUNCTION_ARGUMENTS(arguments) = self.visit(list[0].clone()) {
                        if list.len() != 2 {
                            self.error("Anonymous function's must have 2 arguments: function arguments and a block");
                        }

                        if let Expression::BLOCK(block, _) = list[1].clone() {
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
        self.current_interpreting_location = value.get_location().clone();

        if let Expression::IDENTIFIER(value, _) = value {
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
        self.current_interpreting_location = value.get_location().clone();

        if let Expression::KEY_VALUE(identifier, expression, _) = value {
            Value::KEY_VALUE(identifier, Box::new(self.visit(*expression)))
        } else {
            Value::NULL
        }
    }

    pub fn visit_string(&mut self, value: Expression) -> Value {
        self.current_interpreting_location = value.get_location().clone();

        if let Expression::STRING(value, _) = value {
            Value::STRING(value.resolve(self))
        } else {
            Value::NULL
        }
    }

    pub fn visit_number(&mut self, value: Expression) -> Value {
        self.current_interpreting_location = value.get_location().clone();

        if let Expression::NUMBER(value, _) = value {
            Value::NUMBER(value)
        } else {
            Value::NULL
        }
    }

    pub fn visit_block(&mut self, value: Expression) -> Value {
        self.current_interpreting_location = value.get_location().clone();

        if let Expression::BLOCK(value, _) = value {
            Value::BLOCK(value)
        } else {
            Value::NULL
        }
    }
}