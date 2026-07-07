use super::{TokenStream, expressions::parse_expression};
use crate::{
    Annotation, FunctionDeclaration, FunctionParam, block::parse_block, consume_all_newlines,
    types::parse_type,
};
use anyhow::{Result, anyhow};
use gneurshk_lexer::tokens::Token;

pub fn parse_func_declaration(tokens: &mut TokenStream) -> Result<FunctionDeclaration> {
    // Read annotations
    let mut annotations = vec![];

    while let Some((Token::Annotation(name), _)) = tokens.peek().cloned() {
        tokens.next(); // Consume the token

        // Read the arguments
        let mut args = vec![];

        if let Some((Token::OpenParen, _)) = tokens.peek() {
            tokens.next(); // Consume the opening parenthesis

            // Handle empty argument list
            if let Some((Token::CloseParen, _)) = tokens.peek() {
                tokens.next(); // Consume the closing parenthesis
            } else {
                loop {
                    let arg = parse_expression(tokens)?;
                    args.push(arg);

                    // Check for comma or closing parenthesis
                    match tokens.peek() {
                        Some((Token::Comma, _)) => {
                            tokens.next(); // Consume the comma
                        }
                        Some((Token::CloseParen, _)) => {
                            tokens.next(); // Consume the closing parenthesis
                            break;
                        }
                        _ => {
                            return Err(anyhow!(
                                "Expected a comma or closing parenthesis in the annotation"
                            ));
                        }
                    }
                }
            }
        }

        // Consume all new line tokens
        consume_all_newlines(tokens);

        // Add the annotation to the list of annotations
        annotations.push(Annotation { name, args });
    }

    // Consume the Func token
    match tokens.next() {
        Some((Token::Func, _)) => {}
        _ => return Err(anyhow!("Expected the 'func' keyword")),
    }

    // Consume all new line tokens
    consume_all_newlines(tokens);

    // Read the function name
    let name = match tokens.next() {
        Some((Token::Word(name), _)) => name,
        _ => return Err(anyhow!("Expected the function name")),
    };

    // Consume all new line tokens
    consume_all_newlines(tokens);

    // Read the parameters
    match tokens.next().clone() {
        Some((Token::OpenParen, _)) => {}
        _ => return Err(anyhow!("Expected an opening parenthesis")),
    }

    let mut parameters = vec![];

    loop {
        match tokens.peek().cloned() {
            Some((Token::NewLine, _)) => {
                tokens.next(); // Consume the token
                continue; // Skip to the next token
            }
            Some((Token::CloseParen, _)) => {
                tokens.next(); // Consume the token
                break; // Stop reading parameters
            }
            Some((Token::Word(name), _)) => {
                tokens.next(); // Consume the token

                // Consume all new line tokens
                consume_all_newlines(tokens);

                // Consume the Colon token
                match tokens.next().clone() {
                    Some((Token::Colon, _)) => {}
                    _ => return Err(anyhow!("Expected a colon after the parameter name")),
                }

                // Consume all new line tokens
                consume_all_newlines(tokens);

                // Check for mutability
                let mutable = if let Some((Token::Mut, _)) = tokens.peek().cloned() {
                    tokens.next(); // Consume the token
                    true
                } else {
                    false
                };

                // Consume all new line tokens
                consume_all_newlines(tokens);

                // Read the parameter type
                let data_type = parse_type(tokens)?.unwrap();

                // Consume all new line tokens
                consume_all_newlines(tokens);

                // Check for a default value
                let default_value = match tokens.peek().cloned() {
                    Some((Token::Equal, _)) => {
                        tokens.next(); // Consume the token

                        // Consume all new line tokens
                        consume_all_newlines(tokens);

                        // Parse the default value expression
                        parse_expression(tokens).ok()
                    }
                    _ => None,
                };

                // Consume all new line tokens
                consume_all_newlines(tokens);

                // Consume the comma if it exists
                if let Some((Token::Comma, _)) = tokens.peek().cloned() {
                    tokens.next(); // Consume the token
                }

                // Add the parameter to the list of parameters
                parameters.push(FunctionParam {
                    name: name.to_string(),
                    mutable,
                    data_type,
                    default_value,
                });
            }
            _ => {
                return Err(anyhow!(
                    "Encountered an unexpected token while parsing function parameters"
                ));
            }
        }
    }

    // Consume all new line tokens
    consume_all_newlines(tokens);

    // Parse the return type
    let return_type = match tokens.peek() {
        Some((Token::OpenBrace, _)) => None,
        Some((Token::Arrow, _)) => {
            tokens.next(); // Consume the Arrow token

            // Consume all new line tokens
            consume_all_newlines(tokens);

            // Read the type
            parse_type(tokens)?
        }
        _ => {
            return Err(anyhow!(
                "Missing a colon after the function name or a return type"
            ));
        }
    };

    // Consume all new line tokens
    consume_all_newlines(tokens);

    // Parse the body of the function
    let block = parse_block(tokens)?;

    Ok(FunctionDeclaration {
        annotations,
        name: name.to_string(),
        params: parameters,
        return_type,
        block: Box::new(block),
    })
}

#[cfg(test)]
mod tests {
    use crate::MemberExpressionBase::{self};
    use crate::types::DataType;
    use crate::{
        Annotation, Assignment, BinaryExpression, BinaryOperator, Block, Expression, FloatLit,
        FunctionDeclaration, FunctionParam, Identifier, IntegerLit, Program, Stmt,
        VariableDeclaration, parse,
    };
    use gneurshk_lexer::lex;

    /// Helper function for testing the parse function
    fn lex_then_parse(input: &'static str) -> Program {
        let tokens = lex(input).expect("Failed to lex");

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {e}"),
        }
    }

    #[test]
    #[should_panic]
    fn unfinished_func() {
        lex_then_parse("func");
    }

    #[test]
    #[should_panic]
    fn unfinished_func_name() {
        lex_then_parse("func apple");
    }

    #[test]
    fn return_type_specified() {
        let stmt = lex_then_parse("func apple() -> Int32 { \n var peas = 2 \n }");

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "apple".to_string(),
                    params: vec![],
                    return_type: Some(DataType::Int32),
                    block: Box::new(Block {
                        body: vec![Stmt::VariableDeclaration(VariableDeclaration::Mutable {
                            name: "peas".to_string(),
                            data_type: None,
                            value: Some(Expression::Integer(IntegerLit {
                                value: 2,
                                span: 37..38
                            })),
                        })]
                    }),
                }],
            }
        );
    }

    #[test]
    fn no_return_specified() {
        let stmt = lex_then_parse("func pear() { \n const cucumbers = 8 \n }");

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "pear".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::VariableDeclaration(VariableDeclaration::Constant {
                            name: "cucumbers".to_string(),
                            data_type: None,
                            value: Expression::Integer(IntegerLit {
                                value: 8,
                                span: 34..35
                            }),
                        })]
                    }),
                }],
            }
        );
    }

    #[test]
    fn func_with_params() {
        let stmt = lex_then_parse("func potato(a: Int32, b: Float32) { }");

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "potato".to_string(),
                    params: vec![
                        FunctionParam {
                            name: "a".to_string(),
                            mutable: false,
                            data_type: DataType::Int32,
                            default_value: None,
                        },
                        FunctionParam {
                            name: "b".to_string(),
                            mutable: false,
                            data_type: DataType::Float32,
                            default_value: None,
                        },
                    ],
                    return_type: None,
                    block: Box::new(Block { body: vec![] }),
                }],
            }
        );
    }

    #[test]
    fn func_with_default_params() {
        let stmt = lex_then_parse("func vegetable(a: Int32 = 5, b: Float32 = 3.0) { }");

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "vegetable".to_string(),
                    params: vec![
                        FunctionParam {
                            name: "a".to_string(),
                            mutable: false,
                            data_type: DataType::Int32,
                            default_value: Some(Expression::Integer(IntegerLit {
                                value: 5,
                                span: 26..27
                            })),
                        },
                        FunctionParam {
                            name: "b".to_string(),
                            mutable: false,
                            data_type: DataType::Float32,
                            default_value: Some(Expression::Float(FloatLit {
                                value: 3.0,
                                span: 42..45
                            })),
                        },
                    ],
                    return_type: None,
                    block: Box::new(Block { body: vec![] }),
                }],
            }
        );
    }

    #[test]
    fn func_with_annotations() {
        let stmt = lex_then_parse("@test func egg() { }");

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![Annotation {
                        name: "test".to_string(),
                        args: vec![],
                    }],
                    name: "egg".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block { body: vec![] }),
                }],
            }
        );
    }

    #[test]
    fn func_with_multiple_annotations() {
        let stmt = lex_then_parse("@test @other(1, 2, 3) func ham() { }");

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![
                        Annotation {
                            name: "test".to_string(),
                            args: vec![],
                        },
                        Annotation {
                            name: "other".to_string(),
                            args: vec![
                                Expression::Integer(IntegerLit {
                                    value: 1,
                                    span: 13..14
                                }),
                                Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 16..17
                                }),
                                Expression::Integer(IntegerLit {
                                    value: 3,
                                    span: 19..20
                                }),
                            ],
                        },
                    ],
                    name: "ham".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block { body: vec![] }),
                }],
            }
        );
    }

    #[test]
    fn func_with_mutable_params() {
        let stmt = lex_then_parse("func mutable_params(a: mut Int32, b: Float32) { }");

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "mutable_params".to_string(),
                    params: vec![
                        FunctionParam {
                            name: "a".to_string(),
                            mutable: true,
                            data_type: DataType::Int32,
                            default_value: None,
                        },
                        FunctionParam {
                            name: "b".to_string(),
                            mutable: false,
                            data_type: DataType::Float32,
                            default_value: None,
                        },
                    ],
                    return_type: None,
                    block: Box::new(Block { body: vec![] }),
                }],
            }
        );
    }

    #[test]
    fn many_new_lines() {
        let source = include_str!("../tests/funcs/many_new_lines.iv");
        let stmt = lex_then_parse(source);

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "add".to_string(),
                    params: vec![
                        FunctionParam {
                            name: "a".to_string(),
                            mutable: true,
                            data_type: DataType::Int32,
                            default_value: None,
                        },
                        FunctionParam {
                            name: "b".to_string(),
                            mutable: false,
                            data_type: DataType::Int32,
                            default_value: Some(Expression::Integer(IntegerLit {
                                value: 1,
                                span: 91..92
                            })),
                        },
                    ],
                    return_type: Some(DataType::Int32),
                    block: Box::new(Block {
                        body: vec![Stmt::Assignment(Assignment {
                            value: Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Identifier(Identifier {
                                    name: "a".to_string(),
                                    span: 115..116
                                })),
                                right: Box::new(Expression::Identifier(Identifier {
                                    name: "b".to_string(),
                                    span: 120..121
                                })),
                                operator: BinaryOperator::Add,
                            }),
                            member: MemberExpressionBase::Identifier(Identifier {
                                name: "a".to_string(),
                                span: 115..116
                            })
                        })],
                    }),
                }],
            }
        );
    }
}
