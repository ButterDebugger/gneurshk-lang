use super::{
    expressions::parse_expression, parse_indented_body, StatementResult, Stmt, TokenStream,
};
use crate::lexer::tokens::Token;

pub fn parse_func_declaration(tokens: &mut TokenStream) -> StatementResult {
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
            Some((Token::NewLine | Token::Indent | Token::Dedent, _)) => {
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
                let type_name = match tokens.next().clone() {
                    Some((Token::Word(name), _)) => name,
                    _ => return Err("Expected a parameter type"),
                };

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

                // Add the parameter to the list of parameters
                parameters.push(Stmt::FunctionParam {
                    name: name.to_string(),
                    type_name: type_name.to_string(),
                    default_value,
                });
            }
            _ => {}
        }
    }

    // Parse the return type
    let return_type = match tokens.next().clone() {
        // No return type specified, default to void
        Some((Token::Colon, _)) => "void".to_string(),
        // Return type specified
        Some((Token::Arrow, _)) => match tokens.next().clone() {
            Some((Token::Word(name), _)) => {
                // Expect a colon after the return type
                match tokens.next().clone() {
                    Some((Token::Colon, _)) => {}
                    _ => return Err("Expected a colon after the return type"),
                }

                // Read the return type
                name.to_string()
            }
            _ => return Err("Expected the return type"),
        },
        _ => return Err("Missing a colon after the function name or a return type"),
    };

    // Parse the body of the function
    let body = parse_indented_body(tokens)?;

    Ok(Stmt::FunctionDeclaration {
        name: name.to_string(),
        params: parameters,
        return_type,
        body,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer,
        parser::{parse, Stmt},
    };

    /// Helper function for testing the parse_func_declaration function
    fn lex_then_parse(input: &'static str) -> Vec<Stmt> {
        let tokens = lexer::lex(input).expect("Failed to lex");

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {}", e),
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
        lex_then_parse("func apple() -> int:\n    var peas = 2");
    }

    #[test]
    fn no_return_specified() {
        lex_then_parse("func apple():\n    const cucumbers = 8");
    }
}
