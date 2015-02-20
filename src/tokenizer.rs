#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Token {
    OpenParen,
    CloseParen,
    Ident(String),
    StrTok(String),
}

type LexicalError = &'static str;

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexicalError> {
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
                                None => { return Err("Expected end of string quote"); }
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
                                Some(next) => !("() \n;".contains_char(*next)),
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
    assert_eq!(tokenize(""), Ok(vec!()));
}

#[test]
fn parens_test() {
    assert_eq!(tokenize("()"), Ok(vec!(Token::OpenParen, Token::CloseParen)));
}

#[test]
fn single_identifier_test() {
    assert_eq!(tokenize("test"), Ok(vec!(Token::Ident("test".to_string()))));
}

#[test]
fn single_str_test() {
    assert_eq!(tokenize("\"test\""), Ok(vec!(Token::StrTok("test".to_string()))));
}

#[test]
fn parens_and_identifier_test() {
    assert_eq!(tokenize("(test)"), Ok(vec!(
        Token::OpenParen,
        Token::Ident("test".to_string()),
        Token::CloseParen
    )));
}

#[test]
fn identifier_and_comment_test() {
    assert_eq!(tokenize("test; this is a comment"), Ok(vec!(
        Token::Ident("test".to_string()),
    )));
}
