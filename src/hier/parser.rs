use crate::hier::report;

use crate::hier::expression::Expression;
use crate::hier::interpolated_string::InterpolatedString;
use crate::hier::location::Location;
use crate::hier::token::Token;
use crate::hier::value::Value;

pub struct Parser {
    pub code: Expression,
    current_index: usize,
    tokens: Vec<Token>,
    had_error: bool,
    pub module_reader: fn(String) -> String,
    pub exit_handler: fn() -> !
}

impl Parser {
    pub fn new(tokens: Vec<Token>, module_reader: fn(String) -> String, exit_handler: fn() -> !) -> Self {
        Self {
            code: Expression::NUMBER(0.0, Location::empty()),
            current_index: 0,
            tokens,
            had_error: false,
            module_reader,
            exit_handler
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
                Token::EXCL_MARK(ref location) => {
                    let mut list = Expression::LIST(vec![], Location::new(String::new(), 0, 0));

                    match self.consume() {
                        Token::LEFT_BRACKET(location) => {
                            let loc = (*location).clone();
                            list = Expression::LIST(self.parse_list(), loc);
                        },
                        _ => report(&format!("Expected ( after !, but {} was found.", current_token.clone()), location.clone())
                    }

                    current_list.push(Expression::BLOCK(vec![list], location.clone()));
                },
                Token::LEFT_BRACKET(location) => current_list.push(Expression::LIST(self.parse_list(), location)),
                Token::RIGHT_BRACKET(_) => return current_list,
                Token::LEFT_CURLY(location) => current_list.push(Expression::BLOCK(self.parse_block(), location)),
                Token::RIGHT_CURLY(_) => report("Unexpected }.", (*current_token.get_location()).clone()),
                Token::STRING(string, location) => current_list.push(Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler), location)),
                Token::NUMBER(number, location) => current_list.push(Expression::NUMBER(number.clone(), location)),
                Token::IDENTIFIER(identifier, location) => {
                    let result = self.parse_identifier(identifier, location, &mut current_list, true);
                    current_list.push(result);
                },
                Token::DOT(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        let current_token = self.consume();
                        if let Token::IDENTIFIER(identifier, location) = current_token {
                            current_list.push(Expression::PROPERTY(Box::new(last_expression), identifier.to_string(), location.clone()));
                        } else {
                            report(&format!("Key can only be an identifier, but {} was found.", current_token), (*current_token.get_location()).clone());
                        }
                    } else {
                        report("Dot must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::LEFT_SQUARE(ref location) => {
                    if let Some(last_expression) = current_list.pop() {
                        let current_token = self.consume().clone();

                        let mut key_expression = Expression::NUMBER(0.0, Location::empty());
                        if let Token::LEFT_CURLY(location) = current_token {
                            key_expression = Expression::BLOCK(self.parse_block(), location.clone());
                        } else if let Token::LEFT_BRACKET(location) = current_token {
                            key_expression = Expression::LIST(self.parse_list(), location.clone());
                        } else if let Token::STRING(string, location) = current_token {
                            key_expression = Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler), location.clone());
                        } else if let Token::NUMBER(number, location) = current_token {
                            key_expression = Expression::NUMBER(number.clone(), location.clone());
                        } else if let Token::IDENTIFIER(identifier, location) = current_token {
                            key_expression = Expression::IDENTIFIER(identifier.clone(), location.clone());
                        } else {
                            report(&format!("Token {} is disallowed in subscript.", current_token), (*current_token.get_location()).clone());
                        }

                        let end = self.consume();
                        if let Token::RIGHT_SQUARE(_) = end { } else {
                            report("Subscript must end with ].", (*end.get_location()).clone());
                        }

                        current_list.push(Expression::LIST(vec![Expression::IDENTIFIER("get".to_string(), location.clone()), last_expression, key_expression], location.clone()))
                    } else {
                        report("Subscript must be preceded by a expression.", (current_token.clone().get_location()).clone());
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
                Token::EXCL_MARK(ref location) => {
                    let mut list = Expression::LIST(vec![], Location::new(String::new(), 0, 0));

                    match self.peek() {
                        Token::LEFT_BRACKET(location) => {
                            let loc = (*location).clone();
                            list = Expression::LIST(self.parse_list(), loc);
                        },
                        _ => report(&format!("Expected ( after !, but {} was found.", current_token), location.clone())
                    }

                    current_list.push(Expression::BLOCK(vec![list], location.clone()));
                },
                Token::LEFT_BRACKET(location) => current_list.push(Expression::LIST(self.parse_list(), location)),
                Token::RIGHT_BRACKET(_) => report("Unexpected ).", (*current_token.get_location()).clone()),
                Token::LEFT_CURLY(location) => current_list.push(Expression::BLOCK(self.parse_block(), location)),
                Token::RIGHT_CURLY(_) => return current_list,
                Token::STRING(string, location) => current_list.push(Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler), location)),
                Token::NUMBER(number, location) => current_list.push(Expression::NUMBER(number.clone(), location)),
                Token::IDENTIFIER(identifier, location) => {
                    let result = self.parse_identifier(identifier, location, &mut current_list, false);
                    current_list.push(result);
                },
                Token::DOT(_) => {
                    if let Some(last_expression) = current_list.pop() {
                        self.consume();

                        let current_token = Token::DOT(Location::new("".to_string(),0, 0));

                        if let Token::IDENTIFIER(identifier, location) = current_token {
                            current_list.push(Expression::PROPERTY(Box::new(last_expression), identifier, location));
                        } else {
                            report(&format!("Key can only be an identifier, but {} was found.", current_token), (*current_token.get_location()).clone());
                        }
                    } else {
                        report("Dot must be preceded by a expression.", (*current_token.get_location()).clone());
                    }
                },
                Token::LEFT_SQUARE(ref location) => {
                    if let Some(last_expression) = current_list.pop() {
                        let current_token = self.consume().clone();

                        let mut key_expression = Expression::NUMBER(0.0, Location::empty());
                        if let Token::LEFT_CURLY(location) = current_token {
                            key_expression = Expression::BLOCK(self.parse_block(), location.clone());
                        } else if let Token::LEFT_BRACKET(location) = current_token {
                            key_expression = Expression::LIST(self.parse_list(), location.clone());
                        } else if let Token::STRING(string, location) = current_token {
                            key_expression = Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler), location.clone());
                        } else if let Token::NUMBER(number, location) = current_token {
                            key_expression = Expression::NUMBER(number.clone(), location.clone());
                        } else if let Token::IDENTIFIER(identifier, location) = current_token {
                            key_expression = Expression::IDENTIFIER(identifier.clone(), location.clone());
                        } else {
                            report(&format!("Token {} is disallowed in subscript.", current_token), (*current_token.get_location()).clone());
                        }

                        let end = self.consume();
                        if let Token::RIGHT_SQUARE(_) = end { } else {
                            report("Subscript must end with ].", (*end.get_location()).clone());
                        }

                        current_list.push(Expression::LIST(vec![Expression::IDENTIFIER("get".to_string(), location.clone()), last_expression, key_expression], location.clone()))
                    } else {
                        report("Subscript must be preceded by a expression.", (current_token.clone().get_location()).clone());
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
            Token::EXCL_MARK(_) => { report("Unexpected ).", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::LEFT_BRACKET(location) => Expression::LIST(self.parse_list(), location),
            Token::RIGHT_BRACKET(_) => { report("Unexpected ).", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::LEFT_CURLY(location) => Expression::BLOCK(self.parse_block(), location),
            Token::RIGHT_CURLY(_) => { report("Unexpected }.", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::STRING(string, location) => Expression::STRING(InterpolatedString::construct(string.clone(), Location::empty(), self.module_reader, self.exit_handler), location),
            Token::NUMBER(number, location) => Expression::NUMBER(number.clone(), location),
            Token::IDENTIFIER(identifier, location) => self.parse_identifier(identifier, location, &mut vec![], false),
            Token::DOT(_) => { report("Unexpected ..", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::LEFT_SQUARE(_) => { report("Unexpected [.", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::RIGHT_SQUARE(_) => { report("Unexpected ].", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
            Token::COLON(_) => { report("Unexpected :.", (*current_token.get_location()).clone()); Expression::VALUE(Value::NULL) },
        };

        expression
    }

    fn parse_identifier(&mut self, identifier: String, location: Location, current_list: &mut Vec<Expression>, is_list: bool) -> Expression {
        if identifier == ">" {
            /*
            Parse piping. Pipe is represented using > symbol. When pipe is used, previous list is placed inside next list as first argument.
            This allows for more readable chaining of long commands. For example, instead of
            (print (map (1 2 3) { (+ element 1 }))
            you can write
            (1 2 3) > (map { (+ element 1) }) > (print)
            The piping syntax is converted into the first example so it has the same effect.
            */

            if current_list.len() == 0 && is_list {
                // Current list is empty when the identifier is the first element of the list, which means that it should be a name for function > (more than).
                return Expression::IDENTIFIER(identifier.clone().to_string(), location);
            } else if current_list.len() == 0 && !is_list {
                report("Unexpected pipe operator (>). It should be placed after a list.", Location::empty());
                return Expression::VALUE(Value::NULL);
            }

            let last_expression = current_list[current_list.len() - 1].clone();
            current_list.remove(current_list.len() - 1);

            let next_token = self.consume();

            match next_token {
                Token::LEFT_BRACKET(_) => { }
                _ => report("There must be a list after the pipe operator (>).", next_token.get_location().clone())
            }

            let mut next_expression = self.parse_list();

            next_expression.insert(1, last_expression);

            Expression::LIST(next_expression, location)
        } else {
            if let Token::COLON(_) = self.peek().clone() {
                self.consume();
                let value = self.parse_expression();
                Expression::KEY_VALUE(identifier.to_string().clone(), Box::new(value), location)
            } else {
                Expression::IDENTIFIER(identifier.clone().to_string(), location)
            }
        }
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