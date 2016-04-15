use std::fmt::{Display, Formatter, Error};
use runtime;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Ident(String),
    StrTok(String),
}

impl Display for Token {
    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        return match self {
            &Token::OpenParen  => f.write_str("("),
            &Token::CloseParen => f.write_str(")"),
            &Token::OpenBrace  => f.write_str("["),
            &Token::CloseBrace => f.write_str("]"),
            &Token::Ident(ref s)  => f.write_str(s),
            &Token::StrTok(ref s) => f.write_fmt(format_args!("\"{}\"", s)),
        };
    }
}

pub type LexicalError = runtime::Error;

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexicalError> {
    let mut tokens = vec![];

    let mut iter = input.chars().peekable();

    loop {
        match iter.next() {
            Some(c) =>
                match c {
                    '(' => tokens.push(Token::OpenParen),
                    ')' => tokens.push(Token::CloseParen),
                    '[' => tokens.push(Token::OpenBrace),
                    ']' => tokens.push(Token::CloseBrace),
                    '"' => {
                        let mut s = String::new();

                        loop {
                            match iter.next() {
                                Some('"') => { break; },
                                Some(str_ch) => {
                                    s.push(str_ch);
                                },
                                None => { return Err(LexicalError::from("Expected end of string quote got EOF")); }
                            }
                        }

                        tokens.push(Token::StrTok(s));
                    },
                    ' ' | '\n' => { },
                    ';' => {
                        while match iter.next() {
                            Some('\n') | None => false,
                            Some(_) => true,
                        }{}
                    },
                    _ => {
                        let mut ident_str:String = String::new();

                        ident_str.push(c);

                        loop {
                            let cont = match iter.peek() {
                                Some(next) => !("[]() \n;".contains(*next)),
                                None => false
                            };

                            if !cont {
                                tokens.push(Token::Ident(ident_str));
                                break;
                            }

                            match iter.next() {
                                Some(next_ident) => {
                                    ident_str.push(next_ident);
                                }
                                None => { break }
                            }
                        }
                    }
                },
            None => { break }
        }
    }

    return Ok(tokens);
}

#[test]
fn empty_program_test() {
    assert_eq!(tokenize("").unwrap(), vec!());
}

#[test]
fn parens_test() {
    assert_eq!(tokenize("()").unwrap(), vec!(Token::OpenParen, Token::CloseParen));
}

#[test]
fn braces_test() {
    assert_eq!(tokenize("[]").unwrap(), vec!(Token::OpenBrace, Token::CloseBrace));
}

#[test]
fn single_identifier_test() {
    assert_eq!(tokenize("test").unwrap(), vec!(Token::Ident("test".to_string())));
}

#[test]
fn single_str_test() {
    assert_eq!(tokenize("\"test\"").unwrap(), vec!(Token::StrTok("test".to_string())));
}

#[test]
fn parens_and_identifier_test() {
    assert_eq!(tokenize("(test)").unwrap(), vec!(
        Token::OpenParen,
        Token::Ident("test".to_string()),
        Token::CloseParen
    ));
}

#[test]
fn identifier_and_comment_test() {
    assert_eq!(tokenize("test; this is a comment").unwrap(), vec!(
        Token::Ident("test".to_string()),
    ));
}
