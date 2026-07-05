use crate::{
    Block, Expression, IfStatement, LoopStmt, Stmt, UnaryExpression, UnaryOperator, block::parse_block, expressions::parse_expression,
};
use anyhow::{Result, anyhow};
use gneurshk_lexer::{TokenStream, tokens::Token};

pub fn parse_loop(tokens: &mut TokenStream) -> Result<LoopStmt> {
    // Consume the Loop token
    match tokens.next() {
        Some((Token::Loop, _)) => {}
        _ => return Err(anyhow!("Expected if statement")),
    }

    // Return a loop statement with the parsed block
    Ok(LoopStmt {
        block: Box::new(parse_block(tokens)?),
    })
}

pub fn parse_while_loop(tokens: &mut TokenStream) -> Result<LoopStmt> {
    // Consume the While token
    match tokens.next() {
        Some((Token::While, _)) => {}
        _ => return Err(anyhow!("Expected if statement")),
    }

    // Parse the condition expression
    let condition = parse_expression(tokens)?;

    // Parse the loop body block
    let body = parse_block(tokens)?;

    // Create while loop guard
    let guard = Stmt::IfStatement(IfStatement {
        condition: Box::new(Expression::UnaryExpression(UnaryExpression {
            value: Box::new(condition),
            operator: UnaryOperator::Not,
        })),
        if_block: Box::new(Block {
            body: vec![Stmt::Break],
        }),
        else_statement: None,
    });

    let mut guarded_body = vec![guard];
    guarded_body.extend(body.body);

    Ok(LoopStmt {
        block: Box::new(Block { body: guarded_body }),
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        Block, Expression, FunctionCall, FunctionDeclaration, LoopStmt, Program, Stmt, StringLit,
        parse,
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
    fn a_loop() {
        let input = r#"
func main() {
    loop {
        println("Hello, world!");
    }
}
        "#;

        let program = lex_then_parse(input);

        assert_eq!(
            program,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    annotations: vec![],
                    block: Box::new(Block {
                        body: vec![Stmt::Loop(LoopStmt {
                            block: Box::new(Block {
                                body: vec![Stmt::FunctionCall(FunctionCall {
                                    name: "println".to_string(),
                                    args: vec![Expression::String(StringLit {
                                        value: "Hello, world!".to_string(),
                                        span: 42..57
                                    })],
                                    span: 34..58
                                })],
                            }),
                        })],
                    })
                }]
            }
        );
    }

    #[test]
    fn break_and_continue_statements() {
        let input = r#"
func main() {
    loop {
        break
        continue
    }
}
        "#;

        let program = lex_then_parse(input);

        assert_eq!(
            program,
            Program {
                imports: vec![],
                functions: vec![FunctionDeclaration {
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    annotations: vec![],
                    block: Box::new(Block {
                        body: vec![Stmt::Loop(LoopStmt {
                            block: Box::new(Block {
                                body: vec![Stmt::Break, Stmt::Continue],
                            }),
                        })],
                    })
                }]
            }
        );
    }
}
