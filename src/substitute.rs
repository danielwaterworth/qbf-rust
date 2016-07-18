use std::collections::HashMap;

use problem;
use problem::Expression;
use problem::TRUE;
use problem::FALSE;

struct Substitutions<'r> {
    map: HashMap<*const (), &'r Expression<'r>>
}

fn substitute_end<'r, X>(
        mut subs: Substitutions<'r>,
        expr_ptr: *const (),
        expr: &'r Expression<'r>,
        mut cb: &mut (for<'r1> FnMut(Substitutions<'r1>, &'r1 Expression<'r1>) -> X + 'r)
    ) -> X {
    subs.map.insert(expr_ptr, expr);
    cb(subs, expr)
}

fn substitute_and<'r, X>(
        subs: Substitutions<'r>,
        a: &'r Expression<'r>,
        b: &'r Expression<'r>,
        variable: u64,
        value: bool,
        f: &mut (for<'r1> FnMut(Substitutions<'r1>, &'r1 Expression<'r1>) -> X + 'r)
    ) -> X {
    substitute_inner(subs, a, variable, value, &mut |subs1, expr| {
        match expr {
            &Expression::False =>
                f(subs1, &FALSE),
            &Expression::True => {
                substitute_inner(subs1, b, variable, value, &mut |subs2, expr1| {
                    f(subs2, expr1)
                })
            },
            _ => {
                substitute_inner(subs1, b, variable, value, &mut |subs2, expr1| {
                    problem::and(expr, expr1, |e| {
                        f(subs2, e)
                    })
                })
            }
        }
    })
}

fn substitute_not<'r, X>(
        subs: Substitutions<'r>,
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        f: &mut (for<'r1> FnMut(Substitutions<'r1>, &'r1 Expression<'r1>) -> X + 'r)
    ) -> X {
    substitute_inner(subs, expr, variable, value, &mut |subs1, expr1| {
        problem::not(expr1, |e| {
            f(subs1, e)
        })
    })
}

fn substitute_inner<'r, X>(
        subs: Substitutions<'r>,
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        mut cb: &mut (for<'r1> FnMut(Substitutions<'r1>, &'r1 Expression<'r1>) -> X + 'r)
    ) -> X {
    if !expr.has_var(variable) {
        return cb(subs, expr);
    };

    let expr_ptr = (expr as *const _) as *const ();
    match subs.map.get(&expr_ptr).map(|v| v.clone()) {
        Some(expr1) => {
            return cb(subs, expr1);
        },
        None => {}
    };

    match expr {
        &Expression::True =>
            substitute_end(subs, expr_ptr, expr, cb),
        &Expression::False =>
            substitute_end(subs, expr_ptr, expr, cb),
        &Expression::Var(ref n) => {
            if *n == variable {
                if value {
                    substitute_end(subs, expr_ptr, &TRUE, cb)
                } else {
                    substitute_end(subs, expr_ptr, &FALSE, cb)
                }
            } else {
                substitute_end(subs, expr_ptr, expr, cb)
            }
        },
        &Expression::Not(ref a) => {
            substitute_not(subs, a, variable, value, &mut |subs1, expr1| {
                substitute_end(subs1, expr_ptr, expr1, cb)
            })
        },
        &Expression::And(_, ref a, ref b) => {
            substitute_and(subs, a, b, variable, value, &mut |subs1, expr1| {
                substitute_end(subs1, expr_ptr, expr1, cb)
            })
        }
    }
}

pub fn substitute<'r, F, X>(
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        mut cb: F) -> X
    where F : for<'r1> FnMut(&'r1 Expression<'r1>) -> X {
    let subs = Substitutions {map: HashMap::new()};
    substitute_inner(subs, expr, variable, value, &mut |_, expr1| {
        cb(expr1)
    })
}
