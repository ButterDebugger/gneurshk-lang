use logos::{Lexer, Logos};

#[derive(Debug, Default, Clone)]
pub struct LexerState {
    indent_size: usize,
    previous_indent: usize,
}

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos(extras = LexerState)]
#[logos(skip r"[ \r\f]+")]
#[logos(skip r"#[^\r\n]*")]
pub enum Token {
    #[regex(r"\n+[ \t]*", spacing)]
    _Whitespace,
    _Dedents(usize),

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
    #[token("import")]
    Import,
    #[token("as")]
    As,
    #[token("from")]
    From,

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

fn spacing(lexer: &mut Lexer<Token>) -> Token {
    let size = lexer.slice().trim_start_matches('\n').len();

    // Detect indentation
    if lexer.extras.indent_size == 0 {
        lexer.extras.indent_size = size;
    }

    // Add the appropriate tokens based on indentation
    if size > lexer.extras.previous_indent {
        lexer.extras.previous_indent = size;

        Token::Indent
    } else if size < lexer.extras.previous_indent {
        // Calculate the number of dedents
        let dedents = (lexer.extras.previous_indent - size) / lexer.extras.indent_size;
        lexer.extras.previous_indent = size;

        Token::_Dedents(dedents)
    } else {
        Token::NewLine
    }
}

fn boolean(lexer: &mut Lexer<Token>) -> bool {
    lexer.slice() == "true"
}
