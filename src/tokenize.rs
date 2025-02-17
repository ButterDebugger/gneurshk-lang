use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \r\f]+")]
pub enum Token {
    #[regex(r"\n")]
    NewLine,
    #[regex(r"\t+", lex_indent)]
    Indent(u16),

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
    #[token("func")]
    Func,
    #[token("struct")]
    Struct,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", lex_word)]
    Word(String),
    #[regex("[0-9]+", lex_integer)]
    Integer(isize),
}

fn lex_word(lexer: &mut Lexer<Token>) -> Result<String, ()> {
    Ok(lexer.slice().to_string())
}

fn lex_integer(lexer: &mut Lexer<Token>) -> Result<isize, ()> {
    Ok(lexer.slice().parse::<isize>().unwrap())
}

fn lex_indent(lexer: &mut Lexer<Token>) -> Result<u16, ()> {
    Ok(lexer.slice().len() as u16)
}

/// Takes a string and returns a vector of tokens
/// # Panics
/// Panics if there are any lexing errors
pub fn lex(input: &str) -> Vec<Token> {
    // Create a lexer instance from the input
    let lexer = Token::lexer(&input);

    // Split the input into tokens and panic if there are any errors
    let mut tokens = vec![];
    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => tokens.push(token),
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
}
