use runtime::{Expr,Error};
use parser::parse;
use tokenizer::tokenize;

pub fn read(text:&str) -> Result<Vec<Expr>, Error> {
    return parse(try!(tokenize(text)).iter());
}
