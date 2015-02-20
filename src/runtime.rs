#![feature(io)]

use std::old_io::stdio::print;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use std::collections::HashMap;

pub type RuntimeResult = Result<Expr, &'static str>;

pub struct BuiltInFun<'a> {
    name:&'a str,
    pub fun:fn (scope:&mut Scope, &[Expr]) -> RuntimeResult,
}

impl<'a> PartialEq for BuiltInFun<'a> {
    fn eq(&self, other:&BuiltInFun) -> bool {
        return self.name == other.name;
    }
}

impl<'a> Debug for BuiltInFun<'a> {
    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        return f.write_str(&self.name);
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Expr {
    SExpr(Vec<Expr>),
    Ident(String),
    StrLit(String),
    BuiltInFun(&'static BuiltInFun<'static>),
    Nil
}

#[allow(dead_code)]
fn do_print_builtin(_:&mut Scope, args:&[Expr]) -> RuntimeResult {
    for arg in args.iter() {
        match arg {
            &Expr::SExpr(_) => {
                print("()");
            },
            &Expr::Ident(ref s) => {
                print(s);
            },
            &Expr::StrLit(ref s) => {
                print(s);
            },
            misc => {
                print!("{:?}", misc);
            }
        }
    }

    return Ok(Expr::Nil);
}

fn do_println_builtin(scope:&mut Scope, args:&[Expr]) -> RuntimeResult {
    let res = do_print_builtin(scope, args);
    println!("");
    return res;
}

#[allow(dead_code)]
fn do_set_builtin(scope:&mut Scope, args:&[Expr]) -> RuntimeResult {
    if args.len() != 2 {
        return Err("Invalid number of arguments");
    }

    let name = try!(match args[0] {
        Expr::StrLit(ref s) => Ok(s),
        _ => Err("Invalid argument: name must be string")
    });

    let ref value = args[1];

    scope.defs.insert(name.clone(), value.clone());
    return Ok(value.clone());
}

#[allow(dead_code)]
static BUILTINS : [BuiltInFun<'static>;3] = [
    BuiltInFun{ name: "print", fun: do_print_builtin},
    BuiltInFun{ name: "println", fun: do_println_builtin},
    BuiltInFun{ name: "set", fun: do_set_builtin}
];

pub struct Scope {
    pub defs:HashMap<String, Expr>,
}

impl Scope {
    pub fn new() -> Scope {
        let mut scope = Scope{defs:HashMap::new()};

        scope.load_builtins();

        return scope;
    }

    fn load_builtins(&mut self) {
        for builtin in BUILTINS.iter() {
            self.defs.insert(builtin.name.to_string(), Expr::BuiltInFun(builtin));
        }
    }

    pub fn lookup_ident(&self, s:&str) -> Option<&Expr> {
        return self.defs.get(s);
    }
}

#[test]
fn test_scope_lookup() {
    let mut scope = Scope::new();

    scope.defs.insert("foo".to_string(), Expr::StrLit("bar".to_string()));

    assert_eq!(
        scope.lookup_ident("foo"),
        Some(&Expr::StrLit("bar".to_string()))
    );
}

