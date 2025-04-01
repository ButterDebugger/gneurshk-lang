use super::{
    expressions::parse_expression, parse_indented_body, StatementResult, Stmt, TokenStream,
};
use crate::lexer::tokens::Token;

pub fn parse_func_declaration(tokens: &mut TokenStream) -> StatementResult {
    // Consume the Func token
    match tokens.next() {
        Some(Token::Func) => {}
        _ => return Err("Expected the 'func' keyword"),
    }

    // Read the function name
    let name = match tokens.next() {
        Some(Token::Word(name)) => name,
        _ => return Err("Expected the function name"),
    };

    // Read the parameters
    match tokens.next() {
        Some(Token::OpenParen) => {}
        _ => return Err("Expected an opening parenthesis"),
    }

    let mut parameters = vec![];

    while let Some(&token) = tokens.peek() {
        match token {
            Token::NewLine | Token::Indent | Token::Dedent => {
                tokens.next(); // Consume the token
                continue; // Skip to the next token
            }
            Token::CloseParen => {
                tokens.next(); // Consume the token
                break; // Stop reading parameters
            }
            Token::Word(name) => {
                tokens.next(); // Consume the token

                // Consume the Colon token
                match tokens.next() {
                    Some(Token::Colon) => {}
                    _ => return Err("Expected a colon after the parameter name"),
                }

                // Read the parameter type
                let type_name = match tokens.next() {
                    Some(Token::Word(name)) => name,
                    _ => return Err("Expected the parameter type name"),
                };

                // Check for a default value
                let default_value = match tokens.peek() {
                    Some(Token::Equal) => {
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
    let return_type = match tokens.next() {
        // No return type specified, default to void
        Some(Token::Colon) => "void".to_string(),
        // Return type specified
        Some(Token::Arrow) => match tokens.next() {
            Some(Token::Word(name)) => {
                // Expect a colon after the return type
                match tokens.next() {
                    Some(Token::Colon) => {}
                    _ => return Err("Expected a colon after the return type"),
                }

                // Read the return type
                name.to_string()
            }
            _ => return Err("Expected the return type"),
        },
        _ => return Err("Unexpected token"),
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
    use super::parse_func_declaration;
    use crate::{lexer, parser::Stmt};

    /// Helper function for testing the parse_func_declaration function
    fn lex_then_parse(input: &str) -> Stmt {
        let tokens = lexer::lex(input);

        match parse_func_declaration(&mut tokens.iter().peekable().clone()) {
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
