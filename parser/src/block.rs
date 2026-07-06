use crate::{Block, TokenStream, parse_statement};
use anyhow::{Result, anyhow};
use gneurshk_lexer::tokens::Token;

// TODO: Make this return statement consistent with the rest of the parser
pub fn parse_block(tokens: &mut TokenStream) -> Result<Block> {
    // Consume an optional NewLine token if its present
    if let Some((Token::NewLine, _)) = tokens.peek() {
        tokens.next(); // Consume the new line token
    }

    // Consume the OpenBrace token
    match tokens.next() {
        Some((Token::OpenBrace, _)) => {}
        _ => return Err(anyhow!("Expected opening brace")),
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
            None => return Err(anyhow!("Unexpected end of tokens in indented block")),
            _ => {}
        }

        let statement = parse_statement(tokens)?;
        body.push(statement);
    }

    Ok(Block { body })
}

#[cfg(test)]
mod tests {
    use crate::{Block, FunctionDeclaration, IntegerLit, Program, Stmt, parse};
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
    fn empty_block() {
        let source = include_str!("../tests/block/empty_block.iv");
        let stmt = lex_then_parse(source);

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::Block(Block { body: vec![] })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn single_line_block() {
        let source = include_str!("../tests/block/single_line_block.iv");
        let stmt = lex_then_parse(source);

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::Block(Block {
                            body: vec![Stmt::Integer(IntegerLit {
                                value: 1,
                                span: 20..21
                            })]
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn multiple_line_block() {
        let source = include_str!("../tests/block/multiple_line_block.iv");
        let stmt = lex_then_parse(source);

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::Block(Block {
                            body: vec![Stmt::Integer(IntegerLit {
                                value: 1,
                                span: 28..29
                            })]
                        })],
                    }),
                }],
            }
        );
    }

    #[test]
    fn nested_blocks() {
        let source = include_str!("../tests/block/nested_blocks.iv");
        let stmt = lex_then_parse(source);

        assert_eq!(
            stmt,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    annotations: vec![],
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    block: Box::new(Block {
                        body: vec![Stmt::Block(Block {
                            body: vec![
                                Stmt::Block(Block {
                                    body: vec![Stmt::Block(Block {
                                        body: vec![Stmt::Integer(IntegerLit {
                                            value: 3,
                                            span: 24..25
                                        })]
                                    })]
                                }),
                                Stmt::Block(Block {
                                    body: vec![Stmt::Integer(IntegerLit {
                                        value: 2,
                                        span: 32..33
                                    })]
                                })
                            ]
                        })],
                    }),
                }],
            }
        );
    }
}
