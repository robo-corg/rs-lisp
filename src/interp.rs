use runtime::{Expr, RuntimeThread, RuntimeResult, Error};

fn call_expr(scope:&mut RuntimeThread, expr:&Expr, args:&[Expr]) -> RuntimeResult {
    match expr {
        &Expr::Function(ref builtin) => {
            (builtin.fun)(scope, args)
        },
        _ => {
            return Err(Error::from("Expected function"));
        }
    }
}

fn expand_macro(scope:&mut RuntimeThread, mac:&Expr, args:&[Expr]) -> RuntimeResult {
    match mac {
        &Expr::Macro(ref mac_fun) => {
            (mac_fun.fun)(scope, args)
        },
        _ => {
            return Err(Error::from("Expected macro not {}"));
        }
    }
}

fn eval_expr(scope:&mut RuntimeThread, expr:&Expr) -> RuntimeResult {
    return match expr {
        &Expr::SExpr(ref sub_exprs) => {
            let lead_expr = try!(eval_expr(scope, &sub_exprs[0]));

            match lead_expr {
                Expr::Macro(_) => {
                    let expanded = try!(expand_macro(scope, &lead_expr, &sub_exprs[1..]));
                    return eval_expr(scope, &expanded);
                },
                _ => {
                    let evaled:Vec<Expr> = try!(sub_exprs[1..].iter().map(|sub_expr| eval_expr(scope, sub_expr)).collect());

                    return call_expr(scope, &lead_expr, &evaled);

                }
            }
        },
        &Expr::Ident(ref ident) => match scope.lookup_ident(ident) {
            Some(ref value) => Ok((*value).clone()),
            None => Err(Error::from(format!("Unknown identifier {}", ident)))
        },
        val => Ok(val.clone())
    };
}

pub fn eval(scope:&mut RuntimeThread, exprs:Vec<Expr>) -> RuntimeResult {
    let mut last_expr = Expr::Nil;

    for expr in exprs.iter() {
        last_expr = try!(eval_expr(scope, &expr));
        println!("done");
    }

    return Ok(last_expr);
}

#[test]
fn test_eval_sexpr() {
    let mut scope = RuntimeThread::new();

    let res = eval_expr(
        &mut scope,
        &Expr::SExpr(vec!(
            Expr::Ident("+".to_string()),
            Expr::Integer(3),
            Expr::Integer(1)
        ))
    );

    assert_eq!(
        res.unwrap(),
        Expr::Integer(4)
    );
}

#[test]
#[should_panic]
fn test_lookup_missing() {
    let mut scope = RuntimeThread::new();

    let res = eval_expr(
        &mut scope,
        &Expr::SExpr(vec!(
            Expr::Ident("barf".to_string()),
            Expr::Integer(3),
            Expr::Integer(1)
        ))
    );

    assert_eq!(
        res.unwrap(),
        Expr::Integer(4)
    );
}
