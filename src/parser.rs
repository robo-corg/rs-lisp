use tokenizer::Token;
use std::fmt::Debug;
use std::iter::Peekable;


#[derive(PartialEq)]
#[derive(Debug)]
pub enum Expr {
    SExpr(Vec<Expr>),
    LiteralStr(String)
}

fn parse_expr<'a, 'b, I>(mut iter: I) -> Result<Expr, &'static str> where I: Iterator<Item=(&'b Token)>{
    let openParen = iter.next();
    let closeParen = iter.next();

    return Ok(Expr::SExpr(Vec::new()));
}

fn parse_stmt<'a, 'b, I>(iter: I) -> Result<Expr, &'static str> where I: Iterator<Item=(&'b Token)>{
    let mut piter = iter.peekable();

    loop {
        match piter.peek() {
            Some(&&Token::OpenParen) => {
                return parse_expr(piter.by_ref());
            },
            Some(&&Token::CloseParen) => {},
            Some(&&Token::Ident(_)) => {},
            None => { break; }
        }
    }

    return Ok(Expr::SExpr(Vec::new()));
}


fn parse<'a, 'b, I>(iter: I) -> Result<Vec<Expr>, &'static str> where I: Iterator<Item=(&'b Token)>{
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
    let expected = vec!(Expr::SExpr(Vec::new()));

    assert_eq!(
        parse(vec!(Token::OpenParen, Token::CloseParen).iter()),
        Ok(expected)
    );

}
