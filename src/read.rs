use runtime::Expr;
use parser::parse;
use tokenizer::tokenize;

pub fn read(text:&str) -> Result<Vec<Expr>, &'static str> {
    return parse(tokenize(text).iter());
}
