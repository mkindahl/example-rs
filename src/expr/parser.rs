//! Predictive expression parse for expressions.

use std::clone::Clone;
use super::tokens::{Token, Tokenizer};
use super::tree::ExprTree;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEndOfInput { rule: &'static str },
    UnexpectedToken { token: Token, rule: &'static str },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::UnexpectedEndOfInput { ref rule } => {
                write!(f, "unexpected end of input when parsing {}", rule)
            }
            Error::UnexpectedToken {
                ref token,
                ref rule,
            } => write!(f, "unexpected token '{}' when parsing {}", token, rule),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Parse expression.
///
/// Parse an expression from a string to produce an expression tree.
/// The grammar is given by the following rules:
///
/// expr ::= term (("+" | "-") term)*
/// term ::= factor (("*" | "/") factor)*
/// factor ::= number | variable | "(" expr ")"
///
/// # Returns
///
/// An expression tree
///
/// # Example
///
/// ```
/// # use examples::expr::parse;
/// # use std::collections::HashMap;
/// let tree = parse("10 + 10").unwrap();
/// let map = HashMap::new();
/// assert_eq!(tree.eval(&map), Ok(20.0));
/// ```
pub fn parse(text: &str) -> Result<ExprTree> {
    let mut tokens = Tokenizer::new(text);
    let tree = expr_rule(&mut tokens);
    let result = match tokens.next() {
        None => tree,
        Some(tok) => Err(Error::UnexpectedToken {
            token: tok,
            rule: "expr",
        }),
    };
    result
}

fn expr_rule(tokens: &mut Tokenizer) -> Result<ExprTree> {
    let mut tree = term_rule(tokens)?;
    loop {
        match {
            let tok = tokens.clone().next();
            tok
        } {
            Some(Token::Plus) | Some(Token::Minus) => {
                let tok = tokens.next().expect("expected '+' or '-'");
                let rhs = term_rule(tokens)?;
                match tok {
                    Token::Plus => {
                        tree = ExprTree::Add(Box::new(tree), Box::new(rhs));
                    }
                    Token::Minus => {
                        tree = ExprTree::Sub(Box::new(tree), Box::new(rhs));
                    }
                    tok => {
                        return Err(Error::UnexpectedToken {
                            token: tok,
                            rule: "expr",
                        });
                    }
                }
            }
            _ => break,
        }
    }
    Ok(tree)
}

fn term_rule(tokens: &mut Tokenizer) -> Result<ExprTree> {
    let mut tree = factor_rule(tokens)?;
    loop {
        match {
            let tok = tokens.clone().next();
            tok
        } {
            Some(Token::Star) | Some(Token::Slash) => {
                let tok = tokens.next().expect("expected '*' or '/'");
                let rhs = factor_rule(tokens)?;
                match tok {
                    Token::Star => {
                        tree = ExprTree::Mul(Box::new(tree), Box::new(rhs));
                    }
                    Token::Slash => {
                        tree = ExprTree::Div(Box::new(tree), Box::new(rhs));
                    }
                    tok => {
                        return Err(Error::UnexpectedToken {
                            token: tok,
                            rule: "term",
                        });
                    }
                }
            }
            _ => break,
        }
    }
    Ok(tree)
}

fn factor_rule(tokens: &mut Tokenizer) -> Result<ExprTree> {
    let result = {
        let tok = tokens
            .next()
            .ok_or(Error::UnexpectedEndOfInput { rule: "factor" })?;
        match tok {
            Token::Float(number) => Ok(ExprTree::Float(number)),
            Token::Symbol(name) => Ok(ExprTree::Var(name)),
            Token::Open => {
                let expr = expr_rule(tokens)?;
                match tokens
                    .next()
                    .ok_or(Error::UnexpectedEndOfInput { rule: "factor" })?
                {
                    Token::Close => Ok(expr),
                    tok => Err(Error::UnexpectedToken {
                        token: tok,
                        rule: "factor",
                    }),
                }
            }
            tok => Err(Error::UnexpectedToken {
                token: tok,
                rule: "factor",
            }),
        }
    };
    result
}

#[cfg(test)]
mod tests {
    use super::Error::*;
    use super::ExprTree::*;
    use super::Token;
    use super::{parse, ExprTree};

    fn check(expr: &str, tree: ExprTree) {
        assert_eq!(parse(expr), Ok(tree));
    }

    #[test]
    fn good_parse() {
        assert_eq!(parse("10"), Ok(Float(10.0)));
        check("10+12", Add(Box::new(Float(10.0)), Box::new(Float(12.0))));
        check(
            "10+x",
            Add(Box::new(Float(10.0)), Box::new(Var("x".to_string()))),
        );
        check(
            "10+x*y",
            Add(
                Box::new(Float(10.0)),
                Box::new(Mul(
                    Box::new(Var("x".to_string())),
                    Box::new(Var("y".to_string())),
                )),
            ),
        );
        check(
            "10 + 12 * 20 - 2",
            Sub(
                Box::new(Add(
                    Box::new(Float(10.0)),
                    Box::new(Mul(Box::new(Float(12.0)), Box::new(Float(20.0)))),
                )),
                Box::new(Float(2.0)),
            ),
        );
    }

    #[test]
    fn bad_parse() {
        assert_eq!(
            parse("10 20"),
            Err(UnexpectedToken {
                token: Token::Float(20.0),
                rule: "expr"
            })
        );
        assert_eq!(
            parse("10++"),
            Err(UnexpectedToken {
                token: Token::Plus,
                rule: "factor"
            })
        );
        assert_eq!(parse("10+("), Err(UnexpectedEndOfInput { rule: "factor" }));
    }
}
