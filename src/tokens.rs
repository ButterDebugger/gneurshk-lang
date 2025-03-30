use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \r\f]+")]
pub enum Token {
    #[regex(r"\n[ \t]*", indent)]
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
    #[token("type")]
    Type,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", word)]
    Word(String),
    #[regex("[0-9]+", integer)]
    Integer(isize),
    #[regex(r"true|false", boolean)]
    Boolean(bool),
}

fn word(lexer: &mut Lexer<Token>) -> String {
    lexer.slice().to_string()
}

fn integer(lexer: &mut Lexer<Token>) -> isize {
    lexer.slice().parse::<isize>().unwrap()
}

fn indent(lexer: &mut Lexer<Token>) -> usize {
    lexer.slice()[1..].len()
}

fn boolean(lexer: &mut Lexer<Token>) -> bool {
    lexer.slice() == "true"
}
