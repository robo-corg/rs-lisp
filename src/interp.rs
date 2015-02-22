use runtime::{Expr, Scope, RuntimeResult};

fn call_expr(scope:&mut Scope, expr:&Expr, args:&[Expr]) -> RuntimeResult {
    match expr {
        &Expr::BuiltInFun(builtin) => {
            let fun = builtin.fun;
            fun(scope, args)
        },
        unexpected => {
            return Err(format!("Expected function not {}", unexpected));
        }
    }
}

fn expand_macro(scope:&mut Scope, mac:&Expr, args:&[Expr]) -> RuntimeResult {
    match mac {
        &Expr::Macro(mac_fun) => {
            let fun = mac_fun.fun;
            return fun(scope, args);
        },
        unexpected => {
            return Err(format!("Expected macro not {}", unexpected));
        }
    }
}

fn eval_expr(scope:&mut Scope, expr:&Expr) -> RuntimeResult {
    return match expr {
        &Expr::SExpr(ref sub_exprs) => {
            let lead_expr = try!(eval_expr(scope, &sub_exprs[0]));

            match lead_expr {
                Expr::Macro(_) => {
                    let expanded =try!(expand_macro(scope, &lead_expr, &sub_exprs[1..]));
                    return eval_expr(scope, &expanded);
                },
                _ => {
                    let evaled:Vec<Expr> = try!(sub_exprs[1..].iter().map(|sub_expr| eval_expr(scope, sub_expr)).collect());

                    return call_expr(scope, &lead_expr, &evaled);

                }
            }
        },
        &Expr::Ident(ref ident) => match scope.lookup_ident(ident.as_slice()) {
            Some(ref value) => Ok((*value).clone()),
            None => Err(format!("Unknown identifier {}", ident))
        },
        val => Ok(val.clone())
    };
}

pub fn eval(scope:&mut Scope, exprs:Vec<Expr>) -> RuntimeResult{
    let mut last_expr = Expr::Nil;

    for expr in exprs.iter() {
        last_expr = try!(eval_expr(scope, &expr));
    }

    return Ok(last_expr);
}

#[test]
fn test_eval_sexpr() {
    let mut scope = Scope::new();

    let res = eval_expr(
        &mut scope,
        &Expr::SExpr(vec!(
            Expr::Ident("+".to_string()),
            Expr::Integer(3),
            Expr::Integer(1)
        ))
    );

    assert_eq!(
        res,
        Ok(Expr::Integer(4))
    );
}
