use logos::{Logos, Span, SpannedIter};
use std::iter::Peekable;
use tokens::Token;
pub mod tokens;

pub struct Scanner<'source> {
    lexer: SpannedIter<'source, Token>,
    source: &'source str,
}

impl<'source> Scanner<'source> {
    pub fn new(input: &'source str) -> Result<Self, String> {
        let lexer = Token::lexer(input).spanned();
        let scanner = Scanner {
            lexer: lexer.clone(),
            source: input,
        };

        // Return an error ahead of time if there are any errors
        for (token, span) in lexer.clone() {
            if token.is_err() {
                return Err(format_lexing_error(input, span));
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
            source: self.source,
        }
    }
}

impl core::fmt::Debug for Scanner<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Scanner").finish()
    }
}

impl Iterator for Scanner<'_> {
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((token, span)) = self.lexer.next() {
            match token {
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

/// Formats a lexing error with source code context
fn format_lexing_error(source: &str, span: logos::Span) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut line_start = 0;
    let mut line_number = 1;

    // Find which line the error is on
    for (i, line) in lines.iter().enumerate() {
        let line_end = line_start + line.len();
        if span.start >= line_start && span.start <= line_end {
            line_number = i + 1;
            break;
        }
        line_start = line_end + 1; // +1 for the newline character
    }

    let column = span.start - line_start + 1;
    let error_line = lines.get(line_number - 1).unwrap_or(&"");
    let error_char = source.chars().nth(span.start).unwrap_or('?');

    format!(
        "Lexing error at line {}, column {}:\n\
         {}\n\
         {}^\n\
         Unexpected character: '{}'",
        line_number,
        column,
        error_line,
        " ".repeat(column - 1),
        error_char
    )
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
    fn comments_are_ignored() {
        let tokens = lex(r#"# This is a comment
    var x = 5  # Inline comment
    # Another comment
    var y = 10"#)
        .expect("Failed to lex")
        .map(|(token, _)| token)
        .collect::<Vec<_>>();

        assert_eq!(
            tokens,
            [
                Token::NewLine,
                Token::Var,
                Token::Word("x".to_string()),
                Token::Equal,
                Token::Integer(5),
                Token::NewLine,
                Token::NewLine,
                Token::Var,
                Token::Word("y".to_string()),
                Token::Equal,
                Token::Integer(10),
            ]
        );
    }
}
