use problem;
use problem::Expression as QExp;

use rc_expression::construct;
use rc_expression::with;

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

pub fn simplify<'a, F, X>(
        expr: &'a QExp<'a>,
        f: &mut F) -> X
    where F : for<'b> FnMut(&'b QExp<'b>) -> X
{
    let mut expr1 = construct(expr);

    for var in 0..expr.with_variables(|v| v.len()) {
        let expr2 =
            with(expr1.clone(), &mut |expr2| {
                transform(var, expr2, &mut |expr3| {
                    if expr3.size() < expr2.size() {
                        Some(construct(expr3))
                    } else {
                        None
                    }
                })
            });

        match expr2 {
            Some(expr3) => {
                expr1 = expr3;
            },
            None => {}
        }
    }

    with(expr1, f)
}
