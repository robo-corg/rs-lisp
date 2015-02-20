use runtime::{Expr, Scope, RuntimeResult};

fn call_expr(scope:&mut Scope, expr:&Expr, args:&[Expr]) -> RuntimeResult {
    match expr {
        &Expr::BuiltInFun(builtin) => {
            let fun = builtin.fun;
            fun(scope, args)
        },
        _ => {
            return Err("Expected function");
        }
    }
}

fn eval_expr(scope:&mut Scope, expr:&Expr) -> RuntimeResult {
    return match expr {
        &Expr::SExpr(ref sub_exprs) => {
            let evaled:Vec<Expr> = try!(sub_exprs.iter().map(|sub_expr| eval_expr(scope, sub_expr)).collect());

            return call_expr(scope, &evaled[0], &evaled[1..]);
        },
        &Expr::Ident(ref ident) => match scope.lookup_ident(ident.as_slice()) {
            Some(ref value) => Ok((*value).clone()),
            None => Err("Could not find value for ident")
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
            Expr::Ident("set".to_string()),
            Expr::StrLit("foo".to_string()),
            Expr::StrLit("bananas".to_string())
        ))
    );

    match res {
        Err(msg) => { panic!(msg); }
        _ => {}
    }

    scope.defs.insert("foo".to_string(), Expr::StrLit("bananas".to_string()));

    assert_eq!(
        scope.lookup_ident(&"foo".to_string()),
        Some(&Expr::StrLit("bananas".to_string()))
    );
}
