use super::{StatementResult, Stmt, TokenStream};
use crate::lexer::tokens::Token;

pub fn parse_import(tokens: &mut TokenStream) -> StatementResult {
    // Consume the Import token
    match tokens.next() {
        Some((Token::Import, _)) => {}
        _ => return Err("Expected import statement"),
    }

    // Check if it's importing everything from a module
    if let Some((Token::Multiply, _)) = tokens.peek() {
        tokens.next(); // Consume the token

        match tokens.peek() {
            // NOTE: Example syntax: import * from math
            Some((Token::From, _)) => {
                tokens.next(); // Consume the token

                // Get the module name
                match tokens.next() {
                    Some((Token::Word(module), _)) => {
                        return Ok(Stmt::ImportEverything { module });
                    }
                    _ => return Err("Expected a module name after the 'from' keyword"),
                }
            }
            // NOTE: Example syntax: import * as rng from random
            Some((Token::As, _)) => {
                tokens.next(); // Consume the token

                // Get the alias name
                let alias = match tokens.next() {
                    Some((Token::Word(name), _)) => name,
                    _ => return Err("Expected an alias name after the 'as' keyword"),
                };

                // Expect the 'from' keyword
                match tokens.next() {
                    Some((Token::From, _)) => {}
                    _ => return Err("Expected the 'from' keyword after module alias"),
                }

                // Get the module name
                match tokens.next() {
                    Some((Token::Word(module), _)) => {
                        return Ok(Stmt::ImportModules {
                            modules: vec![(module, Some(alias))],
                        });
                    }
                    _ => return Err("Expected a module name after the 'from' keyword"),
                }
            }
            _ => return Err("Expected the 'from' or 'as' keyword after '*'"),
        }
    }

    // Since we're not importing everything,
    // We'll instead read the individual items to import
    let items = read_import_items(tokens)?;

    // Check if there is a module to import from
    if let Some((Token::From, _)) = tokens.peek() {
        tokens.next(); // Consume the token

        // Get the module name
        match tokens.next() {
            Some((Token::Word(module), _)) => Ok(Stmt::ImportCollection { module, items }),
            _ => Err("Expected a module name after the 'from' keyword"),
        }
    } else {
        // Otherwise import multiple modules
        Ok(Stmt::ImportModules { modules: items })
    }
}

/// Reads a list of import items
/// # Example
/// `sin, cos, sqrt as square_root`
fn read_import_items(
    tokens: &mut TokenStream,
) -> Result<Vec<(String, Option<String>)>, &'static str> {
    let mut items = Vec::new();

    loop {
        match tokens.next() {
            Some((Token::Word(name), _)) => {
                // Check if there's an alias for this item
                let item_alias =
                    if let Some((Token::As, _)) = tokens.peek() {
                        tokens.next(); // Consume the token

                        // Get the alias name
                        match tokens.next() {
                            Some((Token::Word(name), _)) => Some(name),
                            _ => return Err(
                                "Expected an alias for the imported item after the 'as' keyword",
                            ),
                        }
                    } else {
                        None
                    };

                // Add the item to the list of items
                items.push((name, item_alias));
            }
            _ => {
                return Err("Expected import item name");
            }
        }

        // Checks if there are more items to import
        match tokens.peek() {
            Some((Token::Comma, _)) => {
                tokens.next(); // Consume the token
            }
            _ => break,
        }
    }

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;
    use crate::parser::parse;

    /// Helper function for testing the parse_import function
    fn lex_then_parse(input: &'static str) -> Vec<Stmt> {
        let tokens = lexer::lex(input).expect("Failed to lex");

        match parse(&mut tokens.clone()) {
            Ok(result) => result,
            Err(e) => panic!("Parsing error: {}", e),
        }
    }

    #[test]
    fn import_specific_variables() {
        let stmt = lex_then_parse("import sin, cos, sqrt as square_root from math");

        assert_eq!(
            stmt,
            vec![Stmt::ImportCollection {
                module: "math".to_string(),
                items: vec![
                    ("sin".to_string(), None),
                    ("cos".to_string(), None),
                    ("sqrt".to_string(), Some("square_root".to_string())),
                ],
            }]
        );
    }

    #[test]
    fn import_individual_modules() {
        let stmt = lex_then_parse("import os\nimport time as t\nimport * as rng from random");

        assert_eq!(
            stmt,
            vec![
                Stmt::ImportModules {
                    modules: vec![("os".to_string(), None)],
                },
                Stmt::ImportModules {
                    modules: vec![("time".to_string(), Some("t".to_string()))],
                },
                Stmt::ImportModules {
                    modules: vec![("random".to_string(), Some("rng".to_string()))],
                },
            ]
        );
    }

    #[test]
    fn import_multiple_modules() {
        let stmt = lex_then_parse("import os, time as t, random as rng");

        assert_eq!(
            stmt,
            vec![Stmt::ImportModules {
                modules: vec![
                    ("os".to_string(), None),
                    ("time".to_string(), Some("t".to_string())),
                    ("random".to_string(), Some("rng".to_string())),
                ],
            },]
        );
    }

    #[test]
    fn import_everything_from_module() {
        let stmt = lex_then_parse("import * from math");

        assert_eq!(
            stmt,
            vec![Stmt::ImportEverything {
                module: "math".to_string(),
            }]
        );
    }
}
