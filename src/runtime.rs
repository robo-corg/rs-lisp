use std::io;
use std::fmt;
use std::error;
use std::fmt::{Debug, Display, Formatter};
use std::collections::HashMap;
use std::rc::Rc;


use builtin::add_builtins;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    Panic(String)
}

pub type RuntimeResult = Result<Expr, Error>;


impl From<io::Error> for Error {
    fn from(io_error : io::Error) -> Error {
        return Error::IOError(io_error);
    }
}

impl From<&'static str> for Error {
    fn from(s : &'static str) -> Error {
        return Error::Panic(s.to_string());
    }
}

impl From<String> for Error {
    fn from(s : String) -> Error {
        return Error::Panic(s);
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        return match *self {
            Error::IOError(ref err) => err.description(),
            Error::Panic(ref s) => s
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IOError(ref err) => err.cause(),
            Error::Panic(_) => None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        return formatter.write_str(
            &format!("{:?}", self)
        );
    }
}

//trait Function {
    //fn call(scope:&mut RuntimeThread, args:&[Expr]) -> Expr;
//}
//


#[derive(Clone)]
pub struct Function {
    pub name:String,
    pub fun:Rc<Fn(&mut RuntimeThread, &[Expr]) -> RuntimeResult>
}

//impl Function {
    //fn new<F>(name:&str, fun: &'static Rc<F>) -> Function
         //where F : FnMut(&mut RuntimeThread, &[Expr]) -> RuntimeResult
    //{
        //return Function {
            //name: name.to_string(),
            //fun: fun.clone()
        //}
    //}
//}

impl PartialEq for Function {
    fn eq(&self, other:&Function) -> bool {
        return false;
    }
}

impl Debug for Function {
    fn fmt(&self, f:&mut Formatter) -> Result<(), fmt::Error> {
        return f.write_str(&self.name);
    }
}

pub struct BuiltInFun<'a> {
    pub name:&'a str,
    pub fun:fn (scope:&mut RuntimeThread, &[Expr]) -> RuntimeResult,
}

//impl Function for BuiltInFun {
    //fn call(scope:&mut RuntimeThread, args:&[Expr]) -> Expr {
    //}
//}

impl<'a> PartialEq for BuiltInFun<'a> {
    fn eq(&self, other:&BuiltInFun) -> bool {
        return self.name == other.name;
    }
}

impl<'a> Debug for BuiltInFun<'a> {
    fn fmt(&self, f:&mut Formatter) -> Result<(), fmt::Error> {
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
    Function(Function),
    Macro(Function),
    Integer(i64),
    Nil
}

impl From<i64> for Expr {
    fn from(s : i64) -> Expr {
        return Expr::Integer(s);
    }
}

impl Display for Expr {
    fn fmt(&self, f:&mut Formatter) -> Result<(), fmt::Error> {
        return match self {
            &Expr::SExpr(_) => f.write_str("SExpr"),
            &Expr::Ident(_) => f.write_str("Identifier"),
            &Expr::StrLit(ref s) => f.write_fmt(format_args!("String Literal {}", s)),
            &Expr::Function(ref fun) => f.write_fmt(format_args!("function {}", fun.name)),
            &Expr::Macro(ref fun) => f.write_fmt(format_args!("macro {}", fun.name)),
            &Expr::Integer(n) => f.write_fmt(format_args!("Integer {}", n)),
            &Expr::Nil => f.write_str("Nil"),
        };
    }
}

pub struct RuntimeThread {
    stack : Vec<Scope>
}

impl RuntimeThread {
    pub fn new() -> RuntimeThread {
        let mut thread = RuntimeThread{stack:Vec::new()};

        thread.start_scope();
        add_builtins(&mut thread);

        return thread;
    }

    pub fn start_scope(&mut self) {
        self.stack.push(Scope::new());
    }

    pub fn lookup_ident(&self, s:&str) -> Option<&Expr> {
        return self.stack.last().and_then(
            |scope| scope.lookup_ident(s)
        );
    }

    pub fn def(&mut self, name : &str, value : &Expr) {
        self.stack.last_mut().unwrap().def(name, value);
    }
}

pub struct Scope {
    pub defs : HashMap<String, Expr>,
}

impl Scope {
    pub fn new() -> Scope {
        let mut scope = Scope{defs:HashMap::new()};
        return scope;
    }

    pub fn lookup_ident(&self, s:&str) -> Option<&Expr> {
        return self.defs.get(s);
    }

    pub fn def(&mut self, name : &str, value : &Expr) {
        self.defs.insert(name.to_string().clone(), value.clone());
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

