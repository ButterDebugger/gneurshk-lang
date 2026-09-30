use super::TokenStream;
use crate::{ImportCollection, ImportModule, ImportStmt, ImportedSymbol};
use anyhow::{Result, anyhow};
use gneurshk_lexer::tokens::Token;

pub fn parse_import(tokens: &mut TokenStream) -> Result<ImportStmt> {
    // Consume the Import token
    match tokens.next() {
        Some((Token::Import, _)) => {}
        _ => return Err(anyhow!("Expected import statement")),
    }

    // Check if we're importing symbols or a module
    match tokens.peek() {
        Some((Token::Word(module), _)) => {
            let module = module.to_string();

            tokens.next(); // Consume the module name

            // Read optional alias
            let alias = match tokens.peek() {
                Some((Token::As, _)) => {
                    tokens.next(); // Consume the token

                    match tokens.next() {
                        Some((Token::Word(name), _)) => Some(name),
                        _ => return Err(anyhow!("Expected an alias name after 'as' keyword")),
                    }
                }
                _ => None,
            };

            Ok(ImportStmt::Module(ImportModule { module, alias }))
        }
        Some((Token::OpenBrace, _)) => {
            // Parse the symbols inside the braces
            let symbols = parse_import_symbols(tokens)?;

            // Expect the 'from' keyword
            match tokens.next() {
                Some((Token::From, _)) => {}
                _ => return Err(anyhow!("Expected the 'from' keyword after import symbols")),
            }

            // Read the module name
            let module = match tokens.next() {
                Some((Token::Word(name), _)) => name,
                _ => return Err(anyhow!("Expected a module name after 'from' keyword")),
            };

            Ok(ImportStmt::Collection(ImportCollection { module, symbols }))
        }
        _ => Err(anyhow!("Expected a module or symbol name after import")),
    }
}

/// Reads a list of imported names and their optional aliases
///
/// # Example
/// `{ sin, cos, sqrt as square_root }`
fn parse_import_symbols(tokens: &mut TokenStream) -> Result<Vec<ImportedSymbol>> {
    let mut items = Vec::new();

    // Consume the OpenBrace token
    match tokens.next() {
        Some((Token::OpenBrace, _)) => {}
        _ => return Err(anyhow!("Expected opening brace")),
    }

    // Keep appending items until a CloseBrace token is encountered
    let mut expect_comma = false;

    loop {
        match tokens.peek() {
            Some((Token::CloseBrace, _)) => {
                tokens.next(); // Consume the token
                break; // End of the block
            }
            Some((Token::NewLine, _)) => {
                tokens.next(); // Consume the token
                continue; // Skip to the next token
            }
            None => return Err(anyhow!("Unexpected end of tokens inside block")),
            _ => match tokens.next() {
                Some((Token::Comma, _)) => {
                    if expect_comma {
                        expect_comma = false;
                    } else {
                        return Err(anyhow!("Unnecessary or misplaced comma"));
                    }
                }
                Some((Token::Word(name), _)) => {
                    if expect_comma {
                        return Err(anyhow!("Expected a comma before the next item"));
                    }

                    // Check if there's an alias for this item
                    let alias = if let Some((Token::As, _)) = tokens.peek() {
                        tokens.next(); // Consume the token

                        // Get the alias name
                        match tokens.next() {
                            Some((Token::Word(name), _)) => Some(name),
                            _ => {
                                return Err(anyhow!(
                                    "Expected an alias for the imported symbol after the 'as' keyword"
                                ));
                            }
                        }
                    } else {
                        None
                    };

                    // Add the item to the list of items
                    items.push(ImportedSymbol {
                        name: name.to_string(),
                        alias,
                    });

                    // Expect a comma after the item unless it's the last item
                    expect_comma = true;
                }
                _ => return Err(anyhow!("Expected import item name")),
            },
        }
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use crate::{ImportCollection, ImportModule, ImportStmt, ImportedSymbol, Program, parse};
    use gneurshk_lexer::lex;

    /// Helper function for testing the parse function
    fn lex_then_parse(input: &'static str) -> Program {
        let tokens = lex(input).expect("Failed to lex");

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {e}"),
        }
    }

    #[test]
    fn import_collection_of_symbols() {
        let stmt = lex_then_parse("import { sin, cos, sqrt as square_root } from math");

        assert_eq!(
            stmt,
            Program {
                imports: vec![ImportStmt::Collection(ImportCollection {
                    module: "math".to_string(),
                    symbols: vec![
                        ImportedSymbol {
                            name: "sin".to_string(),
                            alias: None
                        },
                        ImportedSymbol {
                            name: "cos".to_string(),
                            alias: None
                        },
                        ImportedSymbol {
                            name: "sqrt".to_string(),
                            alias: Some("square_root".to_string())
                        },
                    ],
                })],
                functions: vec![],
            }
        );
    }

    #[test]
    fn import_collection_of_symbols_with_extra_comma() {
        let stmt = lex_then_parse("import { now, } from time");

        assert_eq!(
            stmt,
            Program {
                imports: vec![ImportStmt::Collection(ImportCollection {
                    module: "time".to_string(),
                    symbols: vec![ImportedSymbol {
                        name: "now".to_string(),
                        alias: None
                    },],
                })],
                functions: vec![],
            }
        );
    }

    #[test]
    fn import_module() {
        let stmt = lex_then_parse("import os");

        assert_eq!(
            stmt,
            Program {
                imports: vec![ImportStmt::Module(ImportModule {
                    module: "os".to_string(),
                    alias: None
                })],
                functions: vec![],
            }
        );
    }

    #[test]
    fn import_module_as_alias() {
        let stmt = lex_then_parse("import time as t");

        assert_eq!(
            stmt,
            Program {
                imports: vec![ImportStmt::Module(ImportModule {
                    module: "time".to_string(),
                    alias: Some("t".to_string())
                })],
                functions: vec![],
            }
        );
    }

    #[test]
    #[should_panic]
    fn import_everything_as_alias_from_module() {
        let _ = lex_then_parse("import * as rng from random");
    }

    #[test]
    #[should_panic]
    fn import_multiple_modules() {
        let _ = lex_then_parse("import os, time as t, random as rng");
    }

    #[test]
    #[should_panic]
    fn import_everything_from_module_without_alias() {
        let _ = lex_then_parse("import * from math");
    }
}
