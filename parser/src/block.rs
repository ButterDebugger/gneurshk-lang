use crate::{StatementResult, Stmt, TokenStream, parse_statement};
use gneurshk_lexer::tokens::Token;

pub fn parse_block(tokens: &mut TokenStream) -> StatementResult {
    // Consume an optional NewLine token if its present
    if let Some((Token::NewLine, _)) = tokens.peek() {
        tokens.next(); // Consume the new line token
    }

    // Consume the OpenBrace token
    match tokens.next() {
        Some((Token::OpenBrace, _)) => {}
        _ => return Err("Expected opening brace"),
    }

    // Consume an optional NewLine token if its present
    if let Some((Token::NewLine, _)) = tokens.peek() {
        tokens.next(); // Consume the new line token
    }

    let mut body = vec![];

    // Keep appending statements until a CloseBrace token is encountered
    loop {
        match tokens.peek() {
            Some((Token::CloseBrace, _)) => {
                tokens.next(); // Consume the token
                break; // End of the block
            }
            Some((Token::NewLine, _)) => {
                tokens.next(); // Consume the token
                continue; // Skip to the next token
            }
            None => return Err("Unexpected end of tokens in indented block"),
            _ => {}
        }

        let statement = parse_statement(tokens)?;
        body.push(statement);
    }

    Ok(Stmt::Block { body })
}

#[cfg(test)]
mod tests {
    use crate::{Program, Stmt, parse};
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
    fn empty_block() {
        let stmt = lex_then_parse("{}").body;

        assert_eq!(stmt, vec![Stmt::Block { body: vec![] }]);
    }

    #[test]
    fn single_line_block() {
        let stmt = lex_then_parse("{ 1 }").body;

        assert_eq!(
            stmt,
            vec![Stmt::Block {
                body: vec![Stmt::Literal { value: 1 }]
            }]
        );
    }

    #[test]
    fn multiple_line_block() {
        let stmt = lex_then_parse("{ \n 1 \n }").body;
        let stmt_start = lex_then_parse("{ \n 1 }").body;
        let stmt_end = lex_then_parse("{ 1 \n }").body;

        assert_eq!(stmt, stmt_start);
        assert_eq!(stmt, stmt_end);

        assert_eq!(
            stmt,
            vec![Stmt::Block {
                body: vec![Stmt::Literal { value: 1 }]
            }]
        );
    }

    #[test]
    fn nested_blocks() {
        let stmt = lex_then_parse("{ { { 3 } } { 2 } }").body;

        assert_eq!(
            stmt,
            vec![Stmt::Block {
                body: vec![
                    Stmt::Block {
                        body: vec![Stmt::Block {
                            body: vec![Stmt::Literal { value: 3 }]
                        }]
                    },
                    Stmt::Block {
                        body: vec![Stmt::Literal { value: 2 }]
                    }
                ]
            }]
        );
    }
}
