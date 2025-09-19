use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(skip r"[ \r\t\f]+")] // Skip whitespace
#[logos(skip r"#[^\r\n]*")] // Skip comments
pub enum Token {
    #[regex(r"(\n|\r\n|;)+")]
    NewLine,

    /// "{"
    #[token("{")]
    OpenBrace,
    /// "}"
    #[token("}")]
    CloseBrace,
    /// "("
    #[token("(")]
    OpenParen,
    /// ")"
    #[token(")")]
    CloseParen,
    /// "["
    #[token("[")]
    OpenBracket,
    /// "]"
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

    // Math operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulus,

    // Bitwise operators
    #[token("|")]
    BitwiseOr,
    #[token("&")]
    BitwiseAnd,
    #[token("^")]
    BitwiseXor,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("~")]
    BitwiseNot,

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

    #[regex(r"and|&&", priority = 20)]
    And,
    #[regex(r"or|\|\|", priority = 20)]
    Or,
    #[regex(r"not|!", priority = 20)]
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
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token("from")]
    From,
    #[token("return")]
    Return,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", word, priority = 1)]
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

fn boolean(lexer: &mut Lexer<Token>) -> bool {
    lexer.slice() == "true"
}
