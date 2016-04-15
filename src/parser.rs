use tokenizer::Token;
use std::iter::Peekable;
use runtime::{Expr, RuntimeResult, Error};

fn parse_sexpr<'a, 'b, I>(piter: &mut Peekable<I>) -> RuntimeResult where I: Iterator<Item=(&'b Token)>{
    let mut exprs = Vec::new();

    // Parse open paren
    let start_paren = piter.next().expect("Unexpected end of input");

    loop {
        match piter.peek() {
            Some(&&Token::CloseParen) => {
                break;
            },
            Some(&&Token::CloseBrace) => {
                break;
            },
            Some(_) => {
                match parse_expr(piter) {
                    Ok(expr) => exprs.push(expr),
                    Err(msg) => return Err(msg)
                }
            },
            None => {
                return Err(Error::Panic("Unexpected end of input".to_string()));
            }
        }
    }

    // Parse close paren
    let close_paren = piter.next().expect("Unexpected end of input");

    match (start_paren, close_paren) {
        (&Token::OpenParen, &Token::CloseParen) |
        (&Token::OpenBrace, &Token::CloseBrace) => {},
        _ => {
            return Err(
                Error::from(format!("Starting {} does not mach closing {}", start_paren, close_paren))
            );
        }
    }

    return Ok(Expr::SExpr(exprs));
}

fn parse_expr<'a, 'b, I>(piter: &mut Peekable<I>) -> RuntimeResult where I: Iterator<Item=(&'b Token)>{
    loop {
        match piter.peek() {
            Some(&&Token::OpenParen) |
            Some(&&Token::OpenBrace) => {
                return parse_sexpr(piter);
            },
            Some(&&Token::Ident(ref s)) => {
                piter.next();
                return Ok(Expr::Ident(s.to_string()));
            },
            Some(&&Token::StrTok(ref s)) => {
                piter.next();
                return Ok(Expr::StrLit(s.to_string()));
            },
            //Some(&ref unexpected) => {
                //return Err(format!("Unexpected token: {}", unexpected));
            //},
            Some(_) => {
                return Err(
                    Error::from("Unexpected token")
                );
            },
            None => { break; }
        }
    }

    return Ok(Expr::Nil);
}


pub fn parse<'a, 'b, I>(iter: I) -> Result<Vec<Expr>, Error> where I: Iterator<Item=(&'b Token)>{
    let mut piter = iter.peekable();
    let mut stmts: Vec<Expr> = Vec::new();

    loop {
        match piter.peek() {
            Some(_) => { },
            None => { break; }
        }

        let stmt = try!(parse_expr(piter.by_ref()));
        stmts.push(stmt);
    }

    return Ok(stmts);
}

#[test]
fn test_parse_empty_s_expr() {
    assert_eq!(
        parse(vec!(Token::OpenParen, Token::CloseParen).iter()).unwrap(),
        vec!(Expr::SExpr(Vec::new()))
    );
}

#[test]
fn test_paren_matching() {
    assert_eq!(
        parse(vec!(
            Token::OpenParen,
            Token::OpenBrace,
            Token::CloseBrace,
            Token::CloseParen).iter()).unwrap(),
        vec!(Expr::SExpr(vec!(Expr::SExpr(vec!()))))
    );
}

#[test]
fn test_parse_s_expr_with_ident() {
    assert_eq!(
        parse(vec!(Token::OpenParen, Token::Ident("print".to_string()), Token::CloseParen).iter()).unwrap(),
        vec!(Expr::SExpr(vec!(Expr::Ident("print".to_string()))))
    );
}
