use std::io;
use std::io::prelude::*;
use runtime::{BuiltInFun, Expr, Scope, RuntimeResult, Error};

fn add_expr(lhs:&Expr, rhs:&Expr) -> RuntimeResult {
    match (lhs, rhs) {
        (&Expr::Integer(lhs_i), &Expr::Integer(rhs_i)) => Ok(Expr::Integer(lhs_i + rhs_i)),
        (_, _) => Err(Error::from(format!("Can't add {} and {}", rhs, lhs))),
    }
}

fn do_add_builtin(_:&mut Scope, args:&[Expr]) -> RuntimeResult {
    if args.len() == 2 {
        return add_expr(&args[0], &args[1]);
    }

    return Err(Error::from(format!("Invalid number of arguments {}", args.len())));

    //let start = Expr::Integer(0);

    //return fold(args.iter(), start, add_expr);
}

#[allow(dead_code)]
fn do_print_builtin(_:&mut Scope, args:&[Expr]) -> RuntimeResult {
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

fn do_println_builtin(scope:&mut Scope, args:&[Expr]) -> RuntimeResult {
    let res = do_print_builtin(scope, args);
    println!("");
    return res;
}

fn do_def_macro_builtin(scope:&mut Scope, args:&[Expr]) -> RuntimeResult {
    if args.len() != 2 {
        return Err(Error::from(format!("Invalid number of arguments got {}", args.len())));
    }

    let name = try!(match args[0] {
        Expr::Ident(ref s) => Ok(s),
        _ => Err(Error::from("Invalid argument: name must be identifier".to_string()))
    });

    let ref value = args[1];

    scope.defs.insert(name.clone(), value.clone());
    return Ok(value.clone());
}

static BUILTIN_FUNS : [BuiltInFun<'static>;3] = [
    BuiltInFun{ name: "+", fun: do_add_builtin},
    BuiltInFun{ name: "print", fun: do_print_builtin},
    BuiltInFun{ name: "println", fun: do_println_builtin},
];

static BUILTIN_MACROS : [BuiltInFun<'static>;1] = [
    BuiltInFun{
        name: "def",
        fun: do_def_macro_builtin,
    },
];

pub fn add_builtins(scope:&mut Scope) {
    for builtin in BUILTIN_MACROS.iter() {
        scope.defs.insert(builtin.name.to_string(), Expr::Macro(builtin));
    }

    for builtin in BUILTIN_FUNS.iter() {
        scope.defs.insert(builtin.name.to_string(), Expr::BuiltInFun(builtin));
    }
}
