extern crate examples;

use examples::expr::eval;
use examples::expr::parser::Error::*;
use examples::expr::tokens::Token;
use examples::expr::tree::Error::*;
use examples::expr::Error::*;
use std::collections::HashMap;

#[test]
fn good_eval() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), 12.0);
    assert_eq!(eval("10", &map), Ok(10.0));
    assert_eq!(eval("10 + 10", &map), Ok(20.0));
    assert_eq!(eval("10 + x", &map), Ok(22.0));
    assert_eq!(eval("x * 10", &map), Ok(120.0));
    assert_eq!(eval("10 - 10", &map), Ok(0.0));
    assert_eq!(eval("10 * 10", &map), Ok(100.0));
    assert_eq!(eval("10 / 10", &map), Ok(1.0));
    assert_eq!(eval("10 + 2 * 3", &map), Ok(16.0));
    assert_eq!(eval("(10 + 2) * 3", &map), Ok(36.0));
    assert_eq!(eval("(10-x)*3", &map), Ok(-6.0));
}

#[test]
fn bad_eval() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), 12.0);
    assert_eq!(
        eval("10 + x + y", &map),
        Err(Eval(NoValue("y".to_string())))
    );
    assert_eq!(
        eval("10 + ", &map),
        Err(Parser(UnexpectedEndOfInput { rule: "factor" }))
    );
    assert_eq!(
        eval("(10 + x", &map),
        Err(Parser(UnexpectedEndOfInput { rule: "factor" }))
    );
    assert_eq!(
        eval("((10 + x) * 2))", &map),
        Err(Parser(UnexpectedToken {
            token: Token::Close,
            rule: "expr"
        }))
    );
    assert_eq!(
        eval("x y", &map),
        Err(Parser(UnexpectedToken {
            token: Token::Symbol("y".to_string()),
            rule: "expr"
        }))
    );
    assert_eq!(
        eval(")10 + x", &map),
        Err(Parser(UnexpectedToken {
            token: Token::Float(10.0),
            rule: "expr"
        }))
    );
}