use std::io;
use std::io::prelude::*;
use runtime::{BuiltInFun, Expr, RuntimeThread, RuntimeResult, Error};

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
    where F : FnMut(i64,&i64) -> i64 {
    let int_args : Vec<i64> = try!(args.iter().map(expr_to_int).collect());

    let val : i64 = i64::from(
        int_args.iter().fold(init, op)
    );

    return Ok(Expr::from(val));
}

macro_rules! def_build_binary_op {
    (  $name:ident, $init:expr, $op:expr  ) => {
        fn $name (_:&mut RuntimeThread, args:&[Expr]) -> RuntimeResult {
            return perform_binary_op(args, $init, $op);
        }
    };
}

def_build_binary_op!(do_add_builtin, 0, |a,b| a + b);
def_build_binary_op!(do_sub_builtin, 0, |a,b| a - b);
def_build_binary_op!(do_mul_builtin, 1, |a,b| a * b);
def_build_binary_op!(do_div_builtin, 1, |a,b| a / b);


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

static BUILTIN_FUNS : [BuiltInFun<'static>;6] = [
    BuiltInFun{ name: "+", fun: do_add_builtin},
    BuiltInFun{ name: "-", fun: do_sub_builtin},
    BuiltInFun{ name: "*", fun: do_mul_builtin},
    BuiltInFun{ name: "/", fun: do_div_builtin},
    BuiltInFun{ name: "display", fun: do_print_builtin},
    BuiltInFun{ name: "println", fun: do_println_builtin},
];

static BUILTIN_MACROS : [BuiltInFun<'static>;1] = [
    BuiltInFun{
        name: "define",
        fun: do_def_macro_builtin,
    },
];

pub fn add_builtins(scope:&mut RuntimeThread) {
    for builtin in BUILTIN_MACROS.iter() {
        scope.def(builtin.name, &Expr::Macro(builtin));
    }

    for builtin in BUILTIN_FUNS.iter() {
        scope.def(builtin.name, &Expr::BuiltInFun(builtin));
    }
}

#[test]
fn test_add_buildin() {
    let actual = do_add_builtin(&mut RuntimeThread::new(), &vec!(Expr::Integer(40), Expr::Integer(1), Expr::Integer(1)));

    assert_eq!(actual.unwrap(), Expr::Integer(42));
}
