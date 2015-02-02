use tokenizer::Token;
use std::fmt::Debug;
use std::iter::Peekable;

trait Expr:Debug + PartialEq {
}

#[derive(PartialEq)]
#[derive(Debug)]
struct SExpr<'x> {
    elements:Vec<Box<Expr + 'x>>,
}

impl<'x> Expr for SExpr<'x> {
    
}

fn parse_s_expr<'a, I>(iter: I) -> SExpr<'a> where I: Iterator<Item=Token>{
    let mut elements = Vec::new();

    return SExpr { elements: elements };
}

fn parse<'a, 'b, I>(iter: I) -> Vec<Box<Expr + 'a>> where I: Iterator<Item=(&'b Token)>{
    let mut piter = iter.peekable();
    let mut exprs: Vec<Box<Expr>> = Vec::new();

    loop {
        match piter.peek() {
            Some(token) => {
                match **token {
                    Token::OpenParen => {}, //exprs.push(Box::new(parse_s_expr(iter))),
                    _ => {}
                }
            },
            None => { break }
        }
    }

    return exprs;
}

//fn parse<'a>(tokens: Vec<Token>) -> Vec<Box<Expr + 'a>> {
    //let mut iter = tokens.iter();
    //return parse_iter(tokens);
//}

#[test]
fn test_parse_empty_s_expr() {
    assert_eq!(
        parse(vec!(Token::OpenParen, Token::CloseParen).iter()),
        vec!(Box::new(SExpr {elements: vec!()}))
    );

}
