use crate::report;

use crate::expression::Expression;
use crate::location::Location;
use crate::token::Token;
use crate::value::Value;

pub struct Parser {
    pub code: Expression,
    current_index: usize,
    tokens: Vec<Token>,
    had_error: bool
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            code: Expression::NUMBER(0.0),
            current_index: 0,
            tokens,
            had_error: false
        }
    }

    /// Returns bool if there was a error.
    pub fn parse(&mut self) -> bool {
        self.code = self.parse_list()[0].clone();

        self.had_error
    }

    pub fn parse_list(&mut self) -> Vec<Expression> {
        let mut current_list: Vec<Expression> = vec![];

        while self.current_index < self.tokens.len() {
            let current_token = self.consume().clone();

            match current_token {
                Token::LEFT_BRACKET(_) => current_list.push(Expression::LIST(self.parse_list())),
                Token::RIGHT_BRACKET(_) => return current_list,
                Token::LEFT_CURLY(_) => current_list.push(Expression::BLOCK(self.parse_block())),
                Token::RIGHT_CURLY(_) => report("Unexpected }.", (*current_token.get_location()).clone()),
                Token::STRING(string, _) => current_list.push(Expression::STRING(string.clone())),
                Token::NUMBER(number, _) => current_list.push(Expression::NUMBER(number.clone())),
                Token::IDENTIFIER(identifier, _) => current_list.push(if let Token::COLON(_) = self.peek().clone() {
                        self.consume();
                        let value = self.parse_expression();
                        Expression::KEY_VALUE(identifier.to_string().clone(), Box::new(value))
                    } else {
                        Expression::IDENTIFIER(identifier.clone().to_string())
                    }),
                Token::DOT(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        let current_token = self.consume();
                        if let Token::IDENTIFIER(identifier, _) = current_token {
                            current_list.push(Expression::PROPERTY(Box::new(last_expression), identifier.to_string()));
                        } else {
                            report(&format!("Key can only be an identifier, but {} was found.", current_token), (*current_token.get_location()).clone());
                        }
                    } else {
                        report("Dot must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::LEFT_SQUARE(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        let current_token = self.consume();

                        let mut key_expression = Expression::NUMBER(0.0);
                        if let Token::LEFT_CURLY(_) = current_token {
                            key_expression = Expression::BLOCK(self.parse_block());
                        } else if let Token::LEFT_BRACKET(_) = current_token {
                            key_expression = Expression::LIST(self.parse_list());
                        } else if let Token::STRING(string, _) = current_token {
                            key_expression = Expression::STRING(string.clone());
                        } else if let Token::NUMBER(number, _) = current_token {
                            key_expression = Expression::NUMBER(number.clone());
                        } else if let Token::IDENTIFIER(identifier, _) = current_token {
                            key_expression = Expression::IDENTIFIER(identifier.clone());
                        } else {
                            report(&format!("Token {} is disallowed in subscript.", current_token), (*current_token.get_location()).clone());
                        }

                        let end = self.consume();
                        if let Token::RIGHT_SQUARE(_) = end { } else {
                            report("Subscript must end with ].", (*end.get_location()).clone());
                        }

                        current_list.push(Expression::LIST(vec![Expression::IDENTIFIER("get".to_string()), last_expression, key_expression]))
                    } else {
                        report("Subscript must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::RIGHT_SQUARE(_) => report("Unexpected ].", (*current_token.get_location()).clone()),
                Token::COLON(_) => report("Unexpected :.", (*current_token.get_location()).clone()),
            }
        }

        current_list
    }

    pub fn parse_block(&mut self) -> Vec<Expression> {
        let mut current_list: Vec<Expression> = vec![];

        while self.current_index < self.tokens.len() {
            let current_token = self.consume().clone();

            match current_token {
                Token::LEFT_BRACKET(_) => current_list.push(Expression::LIST(self.parse_list())),
                Token::RIGHT_BRACKET(_) => report("Unexpected ).", (*current_token.get_location()).clone()),
                Token::LEFT_CURLY(_) => current_list.push(Expression::BLOCK(self.parse_block())),
                Token::RIGHT_CURLY(_) => return current_list,
                Token::STRING(string, _) => current_list.push(Expression::STRING(string.clone())),
                Token::NUMBER(number, _) => current_list.push(Expression::NUMBER(number.clone())),
                Token::IDENTIFIER(identifier, _) => current_list.push(if let Token::COLON(_) = self.peek().clone() {
                        self.consume();
                        let value = self.parse_expression();
                        Expression::KEY_VALUE(identifier.to_string().clone(), Box::new(value))
                    } else {
                        Expression::IDENTIFIER(identifier.to_string().clone())
                    }),
                Token::DOT(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        self.consume();

                        let current_token = Token::DOT(Location::new("".to_string(),0, 0));

                        if let Token::IDENTIFIER(identifier, _) = current_token {
                            current_list.push(Expression::PROPERTY(Box::new(last_expression), identifier));
                        } else {
                            report(&format!("Key can only be an identifier, but {} was found.", current_token), (*current_token.get_location()).clone());
                        }
                    } else {
                        report("Dot must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::LEFT_SQUARE(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        let current_token = self.consume();

                        let mut key_expression = Expression::NUMBER(0.0);
                        if let Token::LEFT_CURLY(_) = current_token {
                            key_expression = Expression::BLOCK(self.parse_block());
                        } else if let Token::LEFT_BRACKET(_) = current_token {
                            key_expression = Expression::LIST(self.parse_list());
                        } else if let Token::STRING(string, _) = current_token {
                            key_expression = Expression::STRING(string.clone());
                        } else if let Token::NUMBER(number, _) = current_token {
                            key_expression = Expression::NUMBER(number.clone());
                        } else if let Token::IDENTIFIER(identifier, _) = current_token {
                            key_expression = Expression::IDENTIFIER(identifier.clone());
                        } else {
                            report(&format!("Token {} is disallowed in subscript.", current_token), (*current_token.get_location()).clone());
                        }

                        let end = self.consume();
                        if let Token::RIGHT_SQUARE(_) = end { } else {
                            report("Subscript must end with ].", (*end.get_location()).clone());
                        }

                        current_list.push(Expression::LIST(vec![Expression::IDENTIFIER("get".to_string()), last_expression, key_expression]))
                    } else {
                        report("Subscript must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::RIGHT_SQUARE(_) => report("Unexpected ].", (*current_token.get_location()).clone()),
                Token::COLON(_) => report("Unexpected :.", (*current_token.get_location()).clone()),
            }
        }

        current_list
    }

    pub fn parse_expression(&mut self) -> Expression {
        let current_token = self.consume().clone();

        let expression = match current_token {
            Token::LEFT_BRACKET(_) => Expression::LIST(self.parse_list()),
            Token::RIGHT_BRACKET(_) => { report("Unexpected ).", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::LEFT_CURLY(_) => Expression::BLOCK(self.parse_block()),
            Token::RIGHT_CURLY(_) => { report("Unexpected }.", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::STRING(string, _) => Expression::STRING(string.clone()),
            Token::NUMBER(number, _) => Expression::NUMBER(number.clone()),
            Token::IDENTIFIER(identifier, _) => if let Token::COLON(_) = self.peek() {
                self.consume();
                let value = self.parse_expression();
                Expression::KEY_VALUE(identifier.to_string().clone(), Box::new(value))
            } else {
                Expression::IDENTIFIER(identifier.to_string().clone())
            },
            Token::DOT(_) => { report("Unexpected ..", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::LEFT_SQUARE(_) => { report("Unexpected [.", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::RIGHT_SQUARE(_) => { report("Unexpected ].", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::COLON(_) => { report("Unexpected :.", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
        };

        expression
    }

    fn consume(&mut self) -> &Token {
        let token = &self.tokens[self.current_index];
        self.current_index += 1;
        token
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current_index]
    }
}