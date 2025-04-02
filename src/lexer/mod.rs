use logos::Logos;
use tokens::Token;
pub mod tokens;

/// Takes a string and returns a vector of tokens
/// # Panics
/// Panics if there are any lexing errors
pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    // Create a lexer instance from the input
    let input = input.to_string() + "\n";
    let lexer = Token::lexer(&input);

    // Split the input into tokens
    // Panic if there are any errors
    // Detect indentation and add Indent and Dedent tokens
    let mut tokens = vec![];
    let mut indent_size = 0;
    let mut previous_indent = 0;
    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => {
                // If the token is a whitespace token, add Indent and Dedent tokens
                if let Token::Whitespace(spacing) = token {
                    if spacing > previous_indent {
                        // If this is the first indentation token, set the indent size
                        if indent_size == 0 {
                            indent_size = spacing;
                        }

                        // Add the indent token if the last token was not a dedent
                        match tokens.last() {
                            Some(Token::Dedent) => {
                                tokens.pop();
                            }
                            None => tokens.push(Token::Indent),
                            _ => tokens.push(Token::Indent),
                        }
                    } else if spacing < previous_indent {
                        // Add the dedent token
                        let dedents = (previous_indent - spacing) / indent_size;

                        for _ in 0..dedents {
                            tokens.push(Token::Dedent)
                        }
                    } else {
                        // Only add an indent after non whitespace tokens
                        // i.e. Indent, Dedent, and NewLine
                        match tokens.last() {
                            Some(Token::NewLine) => {}
                            Some(Token::Indent) => {}
                            Some(Token::Dedent) => {}
                            None => tokens.push(Token::NewLine),
                            _ => tokens.push(Token::NewLine),
                        }
                    }

                    previous_indent = spacing;
                    continue;
                }

                // Otherwise, add the token
                tokens.push(token)
            }
            Err(e) => return Err(format!("Lexing error at {span:?} {e:#?}")),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn unknown_token() {
        lex("`").expect("Failed to lex");
    }

    #[test]
    fn indentation() {
        let tokens = lex(r#"
if true:
    1
    2
    if true:
        3
    4
else:
    if true:
        5"#)
        .expect("Failed to lex");

        assert_eq!(
            tokens,
            [
                Token::NewLine,
                Token::If,
                Token::Boolean(true,),
                Token::Colon,
                Token::Indent,
                Token::Integer(1,),
                Token::NewLine,
                Token::Integer(2,),
                Token::NewLine,
                Token::If,
                Token::Boolean(true,),
                Token::Colon,
                Token::Indent,
                Token::Integer(3,),
                Token::Dedent,
                Token::Integer(4,),
                Token::Dedent,
                Token::Else,
                Token::Colon,
                Token::Indent,
                Token::If,
                Token::Boolean(true,),
                Token::Colon,
                Token::Indent,
                Token::Integer(5,),
                Token::Dedent,
                Token::Dedent,
            ]
        );
    }
}
