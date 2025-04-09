use logos::{Logos, Span, SpannedIter};
use std::iter::Peekable;
use tokens::Token;
pub mod tokens;

pub struct Scanner<'source> {
    lexer: SpannedIter<'source, Token>,
    pending_dedents: usize,
}

impl<'source> Scanner<'source> {
    pub fn new(input: &'source str) -> Result<Self, String> {
        let lexer = Token::lexer(input).spanned();
        let scanner = Scanner {
            lexer: lexer.clone(),
            pending_dedents: 0,
        };

        // Return an error ahead of time if there are any errors
        for (token, span) in lexer.clone() {
            if let Err(e) = token {
                return Err(format!("Lexing error at {span:?} {e:#?}"));
            }
        }

        // Return the scanner
        Ok(scanner)
    }
}

impl Clone for Scanner<'_> {
    fn clone(&self) -> Self {
        Scanner {
            lexer: self.lexer.clone(),
            pending_dedents: self.pending_dedents,
        }
    }
}

impl core::fmt::Debug for Scanner<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Scanner")
            .field("pending_dedents", &self.pending_dedents)
            .finish()
    }
}

impl Iterator for Scanner<'_> {
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return Some((Token::Dedent, 0..0));
        }

        if let Some((token, span)) = self.lexer.next() {
            match token {
                Ok(Token::_Dedents(dedents)) => {
                    self.pending_dedents += dedents - 1;

                    Some((Token::Dedent, span))
                }
                Ok(token) => Some((token, span)),
                Err(_) => panic!("Somehow overlooked a lexing error"),
            }
        } else {
            None
        }
    }
}

/// Takes a string and returns a peekable iterator of tokens
/// # Panics
/// Panics if there are any lexing errors
pub fn lex(input: &str) -> Result<Peekable<Scanner<'_>>, String> {
    // Create a lexer instance from the input
    // Panic ahead of time if there are any errors
    let scanner = Scanner::new(input)?;

    // Return the scanner with the peekable trait
    Ok(scanner.peekable())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn unknown_token() {
        let _ = lex("`").expect("Failed to lex");
    }

    #[test]
    fn indentation_no_gaps() {
        let tokens = lex(r#"
if true:
    1
    2
    if true:
        3
    4
else:
    if true:
        5
"#)
        .expect("Failed to lex")
        .map(|(token, _)| token)
        .collect::<Vec<_>>();

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

    #[test]
    fn indentation_single_gap() {
        let tokens = lex(r#"
if true:
    1

    2
"#)
        .expect("Failed to lex")
        .map(|(token, _)| token)
        .collect::<Vec<_>>();

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
                Token::Dedent,
            ]
        );
    }

    #[test]
    fn indentation_double_gap() {
        let tokens = lex(r#"
if true:
    1


    2
"#)
        .expect("Failed to lex")
        .map(|(token, _)| token)
        .collect::<Vec<_>>();

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
                Token::Dedent,
            ]
        );
    }

    #[test]
    fn indentation_single_starting_gap() {
        let tokens = lex(r#"
if true:

    1
    2
"#)
        .expect("Failed to lex")
        .map(|(token, _)| token)
        .collect::<Vec<_>>();

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
                Token::Dedent,
            ]
        );
    }

    #[test]
    fn indentation_double_starting_gap() {
        let tokens = lex(r#"
if true:


    1
    2
"#)
        .expect("Failed to lex")
        .map(|(token, _)| token)
        .collect::<Vec<_>>();

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
                Token::Dedent,
            ]
        );
    }
}
