use super::{Stmt, TokenStream, expressions::parse_expression};
use crate::types::parse_type;
use anyhow::{Result, anyhow};
use gneurshk_lexer::tokens::Token;

pub fn parse_variable_declaration(tokens: &mut TokenStream) -> Result<Stmt> {
    let mutable = match tokens.next() {
        Some((Token::Var, _)) => true,
        Some((Token::Const, _)) => false,
        _ => return Err(anyhow!("Expected variable declaration")),
    };

    // Read variable name
    let name = match tokens.next() {
        Some((Token::Word(name), _)) => name,
        _ => return Err(anyhow!("Expected variable name")),
    };

    // Check if there is a type
    let data_type = match tokens.peek() {
        Some((Token::Colon, _)) => {
            tokens.next(); // Consume the token

            Some(parse_type(tokens)?)
        }
        _ => None,
    };

    // Check if there is a value
    let init_value = match tokens.peek() {
        Some((Token::Equal, _)) => {
            tokens.next(); // Consume the token

            // Parse the expression
            let value = parse_expression(tokens)?;

            // TODO: Based on the value, determine the type if it wasn't specified

            Some(value)
        }
        _ => {
            // Return an error if there is no type and no value
            if data_type.is_none() {
                return Err(anyhow!("Expected a type or value for the variable"));
            }

            // Otherwise, return no value
            None
        }
    };

    // Return the declaration
    Ok(Stmt::Declaration {
        mutable,
        name: name.to_string(),
        data_type,
        value: init_value,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        BinaryExpression, BinaryOperator, Block, Expression, FunctionDeclaration, IntegerLit,
        Program, Stmt, parse, types::DataType,
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
    fn invalid_variable_declaration() {
        lex_then_parse(
            r#"
func main() {
    var var extra_extra = 0
}
            "#,
        );
    }

    #[test]
    #[should_panic]
    fn unfinished_variable_declaration() {
        lex_then_parse(
            r#"
func main() {
    var
}
            "#,
        );
    }

    #[test]
    #[should_panic]
    fn unfinished_constant_declaration() {
        lex_then_parse(
            r#"
func main() {
    const
}
            "#,
        );
    }

    #[test]
    #[should_panic]
    fn no_type_or_value() {
        let stmt = lex_then_parse(
            r#"
func main() {
    var apple
}
            "#,
        );

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::Declaration {
                            mutable: true,
                            name: "apple".to_string(),
                            data_type: None,
                            value: None
                        }],
                    }),
                }],
            }
        );
    }

    #[test]
    fn has_type_no_value() {
        let stmt = lex_then_parse(
            r#"
func main() {
    var pepper: Int32
}
            "#,
        );

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::Declaration {
                            mutable: true,
                            name: "pepper".to_string(),
                            data_type: Some(DataType::Int32),
                            value: None
                        }],
                    }),
                }],
            }
        );
    }

    #[test]
    fn has_type_and_value() {
        let stmt = lex_then_parse(
            r#"
func main() {
    var potatoes: Int32 = 5
}
            "#,
        );

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::Declaration {
                            mutable: true,
                            name: "potatoes".to_string(),
                            data_type: Some(DataType::Int32),
                            value: Some(Expression::Integer(IntegerLit {
                                value: 5,
                                span: 41..42
                            }))
                        }],
                    }),
                }],
            }
        );
    }

    #[test]
    fn has_value_no_type() {
        let stmt = lex_then_parse(
            r#"
func main() {
    var canned_corn = 2 + 5
}
            "#,
        );

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: DataType::default(),
                    block: Box::new(Block {
                        body: vec![Stmt::Declaration {
                            mutable: true,
                            name: "canned_corn".to_string(),
                            data_type: None,
                            value: Some(Expression::BinaryExpression(BinaryExpression {
                                left: Box::new(Expression::Integer(IntegerLit {
                                    value: 2,
                                    span: 37..38
                                })),
                                right: Box::new(Expression::Integer(IntegerLit {
                                    value: 5,
                                    span: 41..42
                                })),
                                operator: BinaryOperator::Add
                            }))
                        }],
                    }),
                }],
            }
        );
    }
}
