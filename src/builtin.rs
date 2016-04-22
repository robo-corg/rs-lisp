use std::io;
use std::ops::*;
use std::rc::Rc;
use std::io::prelude::*;
use runtime::{Function, Expr, RuntimeThread, RuntimeResult, Error};

macro_rules! _as_ident {
    ( $tok:ident ) => { $tok }
}

fn expr_to_int(expr:&Expr) -> Result<i64, Error> {
    match expr {
        &Expr::Integer(val) => Ok(val),
        _ => Err(Error::from("Could not convert argument"))
    }
}

fn perform_binary_op<F>(args:&[Expr], init:i64, op:F) -> RuntimeResult 
    where F : Fn(i64,i64) -> i64 {
    let int_args : Vec<i64> = try!(args.iter().map(expr_to_int).collect());

    let val : i64 = i64::from(
        int_args.iter().fold(init, |a, &b| op(a,b))
    );

    return Ok(Expr::from(val));
}

fn reg_bin_op(scope:&mut RuntimeThread, name : &str, init : i64, fun : fn(i64, i64) -> i64) 
{
    scope.def(
        name,
        &Expr::Function(Function{
            name: name.to_string(),
            fun: Rc::new(move |_:&mut RuntimeThread, args:&[Expr]| perform_binary_op(args, init,  fun))
        })
    );
}

fn reg_fn(scope:&mut RuntimeThread, name : &str, fun : fn(&mut RuntimeThread, &[Expr]) -> RuntimeResult)
{
    scope.def(
        name,
        &Expr::Function(Function{
            name: name.to_string(),
            fun: Rc::new(fun)
        })
    );
}

fn reg_macro(scope:&mut RuntimeThread, name : &str, fun : fn(&mut RuntimeThread, &[Expr]) -> RuntimeResult)
{
    scope.def(
        name,
        &Expr::Macro(Function{
            name: name.to_string(),
            fun: Rc::new(fun)
        })
    );
}

#[allow(dead_code)]
fn do_print_builtin(_:&mut RuntimeThread, args:&[Expr]) -> RuntimeResult {
    for arg in args.iter() {
        match arg {
            &Expr::SExpr(_) => {
                print!("()");
            },
            &Expr::Ident(ref s) => {
                try!(io::stdout().write(s.as_bytes()));
            },
            &Expr::StrLit(ref s) => {
                try!(io::stdout().write(s.as_bytes()));
            },
            misc => {
                print!("{:?}", misc);
            }
        }
    }

    return Ok(Expr::Nil);
}

fn do_println_builtin(scope:&mut RuntimeThread, args:&[Expr]) -> RuntimeResult {
    let res = do_print_builtin(scope, args);
    println!("");
    return res;
}

fn do_def_macro_builtin(scope:&mut RuntimeThread, args:&[Expr]) -> RuntimeResult {
    if args.len() != 2 {
        return Err(Error::from(format!("Invalid number of arguments got {}", args.len())));
    }

    let name = try!(match args[0] {
        Expr::Ident(ref s) => Ok(s),
        _ => Err(Error::from("Invalid argument: name must be identifier".to_string()))
    });

    let ref value = args[1];

    scope.def(name, value);

    return Ok(value.clone());
}

pub fn add_builtins(scope:&mut RuntimeThread) {
    reg_bin_op(scope, "+", 0, <i64 as Add>::add);
    reg_bin_op(scope, "-", 0, <i64 as Sub>::sub);
    reg_bin_op(scope, "*", 1, <i64 as Mul>::mul);
    reg_bin_op(scope, "/", 1, <i64 as Div>::div);

    reg_fn(scope, "display", do_println_builtin);
    reg_fn(scope, "print", do_println_builtin);
    reg_macro(scope, "define", do_def_macro_builtin);
}

