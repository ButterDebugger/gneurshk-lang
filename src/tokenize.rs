use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \r\f]+")]
pub enum Token {
    #[regex(r"\n[ \t]*", lex_indent)]
    Whitespace(usize),

    // Inserted tokens based on whitespace
    NewLine,
    Indent,
    Dedent,

    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("::")]
    DoubleColon,
    #[token("->")]
    Arrow,
    #[token("=")]
    Equal,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,

    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token(">=")]
    GreaterThanEqual,
    #[token("<=")]
    LessThanEqual,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,

    #[regex(r"and|&&")]
    And,
    #[token(r"or|\|\|")]
    Or,
    #[token(r"not|!")]
    Not,

    #[token("var")]
    Var,
    #[token("const")]
    Const,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("func")]
    Func,
    #[token("struct")]
    Struct,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", lex_word)]
    Word(String),
    #[regex("[0-9]+", lex_integer)]
    Integer(isize),
    #[regex(r"true|false", lex_boolean)]
    Boolean(bool),
}

fn lex_word(lexer: &mut Lexer<Token>) -> String {
    lexer.slice().to_string()
}

fn lex_integer(lexer: &mut Lexer<Token>) -> isize {
    lexer.slice().parse::<isize>().unwrap()
}

fn lex_indent(lexer: &mut Lexer<Token>) -> usize {
    lexer.slice()[1..].len()
}

fn lex_boolean(lexer: &mut Lexer<Token>) -> bool {
    lexer.slice() == "true"
}

/// Takes a string and returns a vector of tokens
/// # Panics
/// Panics if there are any lexing errors
pub fn lex(input: &str) -> Vec<Token> {
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
            Err(e) => {
                panic!("lexer error at {:?}: {:?}", span, e);
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn unknown_token() {
        lex("`");
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
        5"#);

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
