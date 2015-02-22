use runtime::Expr;
use parser::parse;
use tokenizer::tokenize;

pub fn read(text:&str) -> Result<Vec<Expr>, String> {
    return parse(try!(tokenize(text)).iter());
}
