#![feature(core)]
#![feature(collections)]
#![feature(env)]
#![feature(io)]
#![feature(path)]

mod tokenizer;
mod runtime;
mod parser;
mod interp;
mod read;

use std::old_io::File;
use runtime::{Expr, Scope};
use read::read;
use interp::eval;
use std::str::from_utf8;
use std::env::args;

fn eval_file(scope:&mut Scope, filename:&str) -> Result<Expr, &'static str> {
    return match File::open(&Path::new(filename)).read_to_end() {
        Ok(contents_bytes) => match from_utf8(contents_bytes.as_slice()) {
            Ok(contents) => eval(scope, try!(read(contents))),
            Err(_) => Err("Error decoding file")
        },
        Err(_) => Err("Error reading file")
    };
}

fn main() {
    let argv:Vec<String> = args().collect();

    if argv.len() == 1 {
        println!("Usage: {} <file to eval>", argv[0]);
        return;
    }

    let filename = &argv[1];

    match eval_file(&mut Scope::new(), filename.as_slice()) {
        Ok(_) => {},
        Err(msg) => { println!("Error: {}", msg); }
    }
}
