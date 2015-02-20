use tokenizer::Token;
use std::iter::Peekable;
use runtime::{Expr, RuntimeResult};



fn parse_sexpr<'a, 'b, I>(piter: &mut Peekable<I>) -> RuntimeResult where I: Iterator<Item=(&'b Token)>{
    let mut exprs = Vec::new();

    // Parse open paren
    piter.next();

    loop {
        match piter.peek() {
            Some(&&Token::OpenParen) |
            Some(&&Token::Ident(_))  |
            Some(&&Token::StrTok(_)) => {
                match parse_expr(piter) {
                    Ok(expr) => exprs.push(expr),
                    Err(msg) => return Err(msg)
                }
            },
            Some(&&Token::CloseParen) => {
                break;
            },
            None => {
                return Err("Unexpected end of input");
            }
        }
    }

    // Parse close paren
    piter.next();

    return Ok(Expr::SExpr(exprs));
}

fn parse_expr<'a, 'b, I>(piter: &mut Peekable<I>) -> RuntimeResult where I: Iterator<Item=(&'b Token)>{
    loop {
        match piter.peek() {
            Some(&&Token::OpenParen) => {
                return parse_sexpr(piter);
            },
            Some(&&Token::CloseParen) => {},
            Some(&&Token::Ident(ref s)) => {
                piter.next();
                return Ok(Expr::Ident(s.to_string()));
            },
            Some(&&Token::StrTok(ref s)) => {
                piter.next();
                return Ok(Expr::StrLit(s.to_string()));
            },
            None => { break; }
        }
    }

    return Ok(Expr::SExpr(Vec::new()));
}


pub fn parse<'a, 'b, I>(iter: I) -> Result<Vec<Expr>, &'static str> where I: Iterator<Item=(&'b Token)>{
    let mut piter = iter.peekable();
    let mut stmts: Vec<Expr> = Vec::new();

    loop {
        match piter.peek() {
            Some(_) => { },
            None => { break; }
        }

        match parse_expr(piter.by_ref()) {
            Ok(stmt) => stmts.push(stmt),
            Err(e) => return Err(e)
        }
    }

    return Ok(stmts);
}

#[test]
fn test_parse_empty_s_expr() {
    assert_eq!(
        parse(vec!(Token::OpenParen, Token::CloseParen).iter()),
        Ok(vec!(Expr::SExpr(Vec::new())))
    );
}

#[test]
fn test_parse_s_expr_with_ident() {
    assert_eq!(
        parse(vec!(Token::OpenParen, Token::Ident("print".to_string()), Token::CloseParen).iter()),
        Ok(vec!(Expr::SExpr(vec!(Expr::Ident("print".to_string())))))
    );
}
