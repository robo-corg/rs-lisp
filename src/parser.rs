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

fn parse_s_expr<'a, 'b, I>(iter: &I) -> Box<Expr + 'a> where I: Iterator<Item=(&'b Token)>{
    let mut elements = Vec::new();

    return Box::new(SExpr { elements: elements });
}

fn parse<'a, 'b, I>(iter: I) -> Vec<Box<Expr + 'a>> where I: Iterator<Item=(&'b Token)>{
    let mut piter = iter.peekable();
    let mut exprs: Vec<Box<Expr>> = Vec::new();

    loop {
        let tokenResult:Option<Token> = match piter.peek() {
            Some(token) => {
                let borrowedToken:&Token = token;
                Some(*borrowedToken.clone())
            },
            None => None
        };

        match tokenResult {
            Some(token) => {
                match token {
                    Token::OpenParen => exprs.push(parse_s_expr(&piter)),
                    _ => {
                        piter.next();
                    }
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
    let expected:Box<Expr> = Box::new(SExpr {elements: vec!()});

    assert_eq!(
        parse(vec!(Token::OpenParen, Token::CloseParen).iter()),
        vec!(expected)
    );

}
