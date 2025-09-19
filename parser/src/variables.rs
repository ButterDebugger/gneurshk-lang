use super::{StatementResult, Stmt, TokenStream, expressions::parse_expression};
use gneurshk_lexer::tokens::Token;

pub fn parse_variable_declaration(tokens: &mut TokenStream) -> StatementResult {
    let mutable = match tokens.next() {
        Some((Token::Var, _)) => true,
        Some((Token::Const, _)) => false,
        _ => return Err("Expected variable declaration"),
    };

    // Read variable name
    let name = match tokens.next() {
        Some((Token::Word(name), _)) => name,
        _ => return Err("Expected variable name"),
    };

    // Check if there is an equal sign which indicates a value
    let has_value = matches!(tokens.peek(), Some((Token::Equal, _)));

    // If there is a value, parse it
    if has_value {
        tokens.next(); // Consume token

        let value = match parse_expression(tokens) {
            Ok(e) => e,
            _ => return Err("Expected a value for the variable"),
        };

        return Ok(Stmt::Declaration {
            mutable,
            name: name.to_string(),
            value: Some(Box::new(value)),
        });
    }

    Ok(Stmt::Declaration {
        mutable,
        name: name.to_string(),
        value: None,
    })
}

#[cfg(test)]
mod tests {
    use crate::BinaryOperator;
    use crate::Stmt;
    use crate::parse;
    use gneurshk_lexer::lex;

    /// Helper function for testing the parse_variable_declaration function
    fn lex_then_parse(input: &'static str) -> Vec<Stmt> {
        let tokens = lex(input).expect("Failed to lex");

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {e}"),
        }
    }

    #[test]
    #[should_panic]
    fn invalid_variable_declaration() {
        lex_then_parse("var var extra_extra = 0");
    }

    #[test]
    #[should_panic]
    fn unfinished_variable_declaration() {
        lex_then_parse("var");
    }

    #[test]
    #[should_panic]
    fn unfinished_constant_declaration() {
        lex_then_parse("const");
    }

    #[test]
    fn blank_variable_declaration() {
        let stmt = lex_then_parse("var apple");

        assert_eq!(
            stmt,
            vec![Stmt::Declaration {
                mutable: true,
                name: "apple".to_string(),
                value: None
            }]
        );
    }

    #[test]
    fn blank_constant_declaration() {
        let stmt = lex_then_parse("const orange");

        assert_eq!(
            stmt,
            vec![Stmt::Declaration {
                mutable: false,
                name: "orange".to_string(),
                value: None
            }]
        );
    }

    #[test]
    fn literal_variable_declaration() {
        let stmt = lex_then_parse("var green_beans = 2");

        assert_eq!(
            stmt,
            vec![Stmt::Declaration {
                mutable: true,
                name: "green_beans".to_string(),
                value: Some(Box::new(Stmt::Literal { value: 2 }))
            }]
        );
    }

    #[test]
    fn expression_variable_declaration() {
        let stmt = lex_then_parse("var canned_corn = 2 + 5");

        assert_eq!(
            stmt,
            vec![Stmt::Declaration {
                mutable: true,
                name: "canned_corn".to_string(),
                value: Some(Box::new(Stmt::BinaryExpression {
                    left: Box::new(Stmt::Literal { value: 2 }),
                    right: Box::new(Stmt::Literal { value: 5 }),
                    operator: BinaryOperator::Add
                }))
            }]
        );
    }
}
