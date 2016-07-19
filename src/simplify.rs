use problem;
use problem::Expression as QExp;

use substitute::substitute;

fn transform<'a, F, X>(
        var: u32,
        expr: &'a QExp<'a>,
        f: &mut F) -> X
    where F : for<'b> FnMut(&'b QExp<'b>) -> X
{
    substitute(expr, var, true, |true_expr| {
        substitute(expr, var, false, |false_expr| {
            let var_expr = QExp::Var(var);
            let var_expr_ = QExp::Not(&var_expr);
            problem::and(&var_expr, true_expr, |a| {
                problem::and(&var_expr_, false_expr, |b| {
                    problem::or(a, b, |e| {
                        f(e)
                    })
                })
            })
        })
    })
}

fn simplify_once<'a, F, X>(
        var: u32,
        expr: &'a QExp<'a>,
        f: &mut F) -> X
    where F : for<'b> FnMut(&'b QExp<'b>) -> X
{
    let outcome =
        transform(var, expr, &mut |expr1| {
            if expr1.size() < expr.size() {
                Some(f(expr1))
            } else {
                None
            }
        });

    match outcome {
        Some(x) => x,
        None => f(expr)
    }
}

pub fn simplify_inner<'a, X>(
        start_at: u32,
        expr: &'a QExp<'a>,
        f: &mut (for<'b> FnMut(&'b QExp<'b>) -> X + 'a)) -> X {
    if start_at < expr.with_variables(|vars| vars.len()) {
        simplify_once(start_at, expr, &mut |expr1| {
            simplify_inner(start_at + 1, expr1, f)
        })
    } else {
        f(expr)
    }
}

pub fn simplify<'a, F, X>(
        expr: &'a QExp<'a>,
        f: &mut F) -> X
    where F : for<'b> FnMut(&'b QExp<'b>) -> X
{
    simplify_inner(0, expr, f)
}
