mod tokenizer;
mod runtime;
mod parser;
mod interp;
mod read;
mod builtin;

#[allow(unused_imports)]
use std::io;
use std::io::prelude::*;
use std::fs::File;
#[allow(unused_imports)]
use runtime::{Expr, Scope, RuntimeResult};
use read::read;
use interp::eval;
#[allow(unused_imports)]
use std::str::from_utf8;
use std::env::args;

fn eval_file(scope:&mut Scope, filename:&str) -> RuntimeResult {
    let mut buffer = String::new();
    let mut file = try!(File::open(filename));

    try!(file.read_to_string(&mut buffer));

    return eval(
        scope,
        try!(read(&buffer))
    );
}

fn main() {
    let argv:Vec<String> = args().collect();

    if argv.len() == 1 {
        println!("Usage: {} <file to eval>", argv[0]);
        return;
    }

    let filename = &argv[1];

    match eval_file(&mut Scope::new(), filename) {
        Ok(_) => {},
        Err(msg) => { println!("Error: {}", msg); }
    }
}
