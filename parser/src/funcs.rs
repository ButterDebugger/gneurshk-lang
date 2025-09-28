use super::{StatementResult, Stmt, TokenStream, expressions::parse_expression};
use crate::{
    Annotation, FunctionParam,
    block::parse_block,
    types::{DataType, parse_type},
};
use gneurshk_lexer::tokens::Token;

pub fn parse_func_declaration(tokens: &mut TokenStream) -> StatementResult {
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
                            return Err(
                                "Expected a comma or closing parenthesis in the annotation",
                            );
                        }
                    }
                }
            }
        }

        // Consume a NewLine token if its present
        if let Some((Token::NewLine, _)) = tokens.peek() {
            tokens.next(); // Consume the token
        }

        // Add the annotation to the list of annotations
        annotations.push(Annotation { name, args });
    }

    // Consume the Func token
    match tokens.next() {
        Some((Token::Func, _)) => {}
        _ => return Err("Expected the 'func' keyword"),
    }

    // Read the function name
    let name = match tokens.next() {
        Some((Token::Word(name), _)) => name,
        _ => return Err("Expected the function name"),
    };

    // Read the parameters
    match tokens.next().clone() {
        Some((Token::OpenParen, _)) => {}
        _ => return Err("Expected an opening parenthesis"),
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

                // Consume the Colon token
                match tokens.next().clone() {
                    Some((Token::Colon, _)) => {}
                    _ => return Err("Expected a colon after the parameter name"),
                }

                // Read the parameter type
                let data_type = parse_type(tokens)?;

                // Check for a default value
                let default_value = match tokens.peek().cloned() {
                    Some((Token::Equal, _)) => {
                        tokens.next(); // Consume the token

                        match parse_expression(tokens) {
                            Ok(e) => Some(Box::new(e)),
                            _ => None,
                        }
                    }
                    _ => None,
                };

                // Consume the comma if it exists
                if let Some((Token::Comma, _)) = tokens.peek().cloned() {
                    tokens.next(); // Consume the token
                }

                // Add the parameter to the list of parameters
                parameters.push(FunctionParam {
                    name: name.to_string(),
                    data_type,
                    default_value,
                });
            }
            _ => {}
        }
    }

    // Parse the return type
    let return_type = match tokens.peek() {
        Some((Token::OpenBrace, _)) => DataType::default(),
        Some((Token::Arrow, _)) => {
            tokens.next(); // Consume the Arrow token

            // Read the type
            parse_type(tokens)?
        }
        _ => return Err("Missing a colon after the function name or a return type"),
    };

    // Parse the body of the function
    let body = parse_block(tokens)?;

    Ok(Stmt::FunctionDeclaration {
        annotations,
        name: name.to_string(),
        params: parameters,
        return_type,
        block: Box::new(body),
    })
}

#[cfg(test)]
mod tests {
    use crate::types::DataType;
    use crate::{Annotation, FunctionParam, Program, Stmt, parse};
    use gneurshk_lexer::lex;

    /// Helper function for testing the parse_func_declaration function
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
                functions: vec![Stmt::FunctionDeclaration {
                    annotations: vec![],
                    name: "apple".to_string(),
                    params: vec![],
                    return_type: DataType::Int32,
                    block: Box::new(Stmt::Block {
                        body: vec![Stmt::Declaration {
                            mutable: true,
                            name: "peas".to_string(),
                            data_type: None,
                            value: Some(Box::new(Stmt::Integer { value: 2 })),
                        }]
                    }),
                }],
                body: vec![],
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
                functions: vec![Stmt::FunctionDeclaration {
                    annotations: vec![],
                    name: "pear".to_string(),
                    params: vec![],
                    return_type: DataType::default(),
                    block: Box::new(Stmt::Block {
                        body: vec![Stmt::Declaration {
                            mutable: false,
                            name: "cucumbers".to_string(),
                            data_type: None,
                            value: Some(Box::new(Stmt::Integer { value: 8 })),
                        }]
                    }),
                }],
                body: vec![],
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
                functions: vec![Stmt::FunctionDeclaration {
                    annotations: vec![],
                    name: "potato".to_string(),
                    params: vec![
                        FunctionParam {
                            name: "a".to_string(),
                            data_type: DataType::Int32,
                            default_value: None,
                        },
                        FunctionParam {
                            name: "b".to_string(),
                            data_type: DataType::Float32,
                            default_value: None,
                        },
                    ],
                    return_type: DataType::default(),
                    block: Box::new(Stmt::Block { body: vec![] }),
                }],
                body: vec![],
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
                functions: vec![Stmt::FunctionDeclaration {
                    annotations: vec![],
                    name: "vegetable".to_string(),
                    params: vec![
                        FunctionParam {
                            name: "a".to_string(),
                            data_type: DataType::Int32,
                            default_value: Some(Box::new(Stmt::Integer { value: 5 })),
                        },
                        FunctionParam {
                            name: "b".to_string(),
                            data_type: DataType::Float32,
                            default_value: Some(Box::new(Stmt::Float { value: 3.0 })),
                        },
                    ],
                    return_type: DataType::default(),
                    block: Box::new(Stmt::Block { body: vec![] }),
                }],
                body: vec![],
            }
        );
    }

    #[test]
    fn func_with_annotations() {
        let stmt = lex_then_parse("@test func egg() { }");
        let stmt_with_newline = lex_then_parse("@test \n func egg() { }");

        assert_eq!(stmt, stmt_with_newline);

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![Stmt::FunctionDeclaration {
                    annotations: vec![Annotation {
                        name: "test".to_string(),
                        args: vec![],
                    }],
                    name: "egg".to_string(),
                    params: vec![],
                    return_type: DataType::default(),
                    block: Box::new(Stmt::Block { body: vec![] }),
                }],
                body: vec![],
            }
        );
    }

    #[test]
    fn func_with_multiple_annotations() {
        let stmt = lex_then_parse("@test @other(1, 2, 3) func ham() { }");
        let stmt_with_newline = lex_then_parse("@test \n @other(1, 2, 3) \n func ham() { }");

        assert_eq!(stmt, stmt_with_newline);

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![Stmt::FunctionDeclaration {
                    annotations: vec![
                        Annotation {
                            name: "test".to_string(),
                            args: vec![],
                        },
                        Annotation {
                            name: "other".to_string(),
                            args: vec![
                                Stmt::Integer { value: 1 },
                                Stmt::Integer { value: 2 },
                                Stmt::Integer { value: 3 },
                            ],
                        },
                    ],
                    name: "ham".to_string(),
                    params: vec![],
                    return_type: DataType::default(),
                    block: Box::new(Stmt::Block { body: vec![] }),
                }],
                body: vec![],
            }
        );
    }
}
