use super::{expressions::parse_expression, StatementResult, Stmt, TokenStream};
use crate::lexer::tokens::Token;

pub fn parse_variable_declaration(tokens: &mut TokenStream) -> StatementResult {
    let mutable = match tokens.next() {
        Some(Token::Var) => true,
        Some(Token::Const) => false,
        _ => return Err("Expected variable declaration"),
    };

    // Read variable name
    let name = match tokens.next() {
        Some(Token::Word(name)) => name,
        _ => return Err("Expected variable name"),
    };

    let has_value = match tokens.peek() {
        Some(Token::Equal) => true,
        _ => false,
    };

    if has_value {
        tokens.next(); // Consume token

        let value = match parse_expression(tokens) {
            Ok(e) => e,
            _ => return Err("Expected an expression"),
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
    use super::parse_variable_declaration;
    use crate::{
        lexer,
        parser::{Operator, Stmt},
    };

    /// Helper function for testing the parse_variable_declaration function
    fn lex_then_parse(input: &str) -> Stmt {
        let tokens = lexer::lex(input);

        match parse_variable_declaration(&mut tokens.iter().peekable().clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {}", e),
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
            Stmt::Declaration {
                mutable: true,
                name: "apple".to_string(),
                value: None
            }
        );
    }

    #[test]
    fn blank_constant_declaration() {
        let stmt = lex_then_parse("const orange");

        assert_eq!(
            stmt,
            Stmt::Declaration {
                mutable: false,
                name: "orange".to_string(),
                value: None
            }
        );
    }

    #[test]
    fn literal_variable_declaration() {
        let stmt = lex_then_parse("var green_beans = 2");

        assert_eq!(
            stmt,
            Stmt::Declaration {
                mutable: true,
                name: "green_beans".to_string(),
                value: Some(Box::new(Stmt::Literal { value: 2 }))
            }
        );
    }

    #[test]
    fn expression_variable_declaration() {
        let stmt = lex_then_parse("var canned_corn = 2 + 5");

        assert_eq!(
            stmt,
            Stmt::Declaration {
                mutable: true,
                name: "canned_corn".to_string(),
                value: Some(Box::new(Stmt::BinaryExpression {
                    left: Box::new(Stmt::Literal { value: 2 }),
                    right: Box::new(Stmt::Literal { value: 5 }),
                    operator: Operator::Add
                }))
            }
        );
    }
}
