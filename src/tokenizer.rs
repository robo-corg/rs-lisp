#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    OpenParen,
    CloseParen,
    Ident(String),
    StrTok(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut iter = input.chars().peekable();

    loop {
        match iter.next() {
            Some(c) =>
                match c {
                    '(' => tokens.push(Token::OpenParen),
                    ')' => tokens.push(Token::CloseParen),
                    '"' => {
                        let mut s = String::new();

                        loop {
                            match iter.next() {
                                Some('"') => { break; },
                                Some(str_ch) => {
                                    s.push(str_ch);
                                },
                                None => { panic!("Expected end of string quote"); }
                            }
                        }

                        tokens.push(Token::StrTok(s));
                    },
                    ' ' | '\n' => { },
                    _ => {
                        let mut ident_str:String = String::new();

                        ident_str.push(c);

                        loop {
                            let cont = match iter.peek() {
                                Some(next) => !("() \n".contains_char(*next)),
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

    return tokens;
}

#[test]
fn empty_program_test() {
    assert_eq!(tokenize(""), []);
}

#[test]
fn parens_test() {
    assert_eq!(tokenize("()"), [Token::OpenParen, Token::CloseParen]);
}

#[test]
fn single_identifier_test() {
    assert_eq!(tokenize("test"), [Token::Ident("test".to_string())]);
}

#[test]
fn single_str_test() {
    assert_eq!(tokenize("\"test\""), [Token::StrTok("test".to_string())]);
}

#[test]
fn parens_and_identifier_test() {
    assert_eq!(tokenize("(test)"), [
        Token::OpenParen,
        Token::Ident("test".to_string()),
        Token::CloseParen
    ]);
}

