#![feature(io)]

use std::old_io::stdio::print;
use std::fmt::{Debug, Display, Formatter, Error};
use std::collections::HashMap;

use builtin::add_builtins;

pub type RuntimeResult = Result<Expr, String>;

pub struct BuiltInFun<'a> {
    pub name:&'a str,
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
    Macro(&'static BuiltInFun<'static>),
    Integer(i64),
    Nil
}

impl Display for Expr {
    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        return match self {
            &Expr::SExpr(_) => f.write_str("SExpr"),
            &Expr::Ident(_) => f.write_str("Identifier"),
            &Expr::StrLit(ref s) => f.write_fmt(format_args!("String Literal {}", s)),
            &Expr::BuiltInFun(fun) => f.write_fmt(format_args!("function {}", fun.name)),
            &Expr::Macro(fun) => f.write_fmt(format_args!("macro {}", fun.name)),
            &Expr::Integer(n) => f.write_fmt(format_args!("Integer {}", n)),
            &Expr::Nil => f.write_str("Nil"),
        };
    }
}

pub struct Scope {
    pub defs:HashMap<String, Expr>,
}

impl Scope {
    pub fn new() -> Scope {
        let mut scope = Scope{defs:HashMap::new()};

        add_builtins(&mut scope);

        return scope;
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

