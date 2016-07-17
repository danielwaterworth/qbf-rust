use std::collections::HashMap;
use std::hash::Hash;

use problem;
use problem::Expression;
use problem::TRUE;
use problem::FALSE;

struct Substitutions<'r> {
    map: HashMap<*const (), &'r Expression<'r>>
}

fn get_clone<K, V>(m: &HashMap<K, V>, k: &K) -> Option<V>
    where K: Eq, K: Hash, V:Clone {
    match m.get(k) {
        Some(v) => Some(v.clone()),
        None => None
    }
}

fn substitute_end<'r, F, X>(
        mut subs: Substitutions<'r>,
        expr_ptr: *const (),
        expr: &'r Expression<'r>,
        cb: F) -> X
    where F : for<'r1> FnOnce(Substitutions<'r1>, &'r1 Expression<'r1>) -> X {
    subs.map.insert(expr_ptr, expr);
    cb(subs, expr)
}

fn substitute_and<'r, F, X>(
        subs: Substitutions<'r>,
        a: &'r Expression<'r>,
        b: &'r Expression<'r>,
        variable: u64,
        value: bool,
        f: F) -> X
    where F : for<'r1> FnOnce(Substitutions<'r1>, &'r1 Expression<'r1>) -> X {
    substitute_inner(subs, a, variable, value, move |subs1, expr| {
        match expr {
            &Expression::False =>
                f(subs1, &FALSE),
            &Expression::True => {
                substitute_inner(subs1, b, variable, value, move |subs2, expr1| {
                    f(subs2, expr1)
                })
            },
            _ => {
                substitute_inner(subs1, b, variable, value, move |subs2, expr1| {
                    match expr1 {
                        &Expression::False => f(subs2, &FALSE),
                        &Expression::True => f(subs2, expr),
                        _ => {
                            let e = problem::and(expr, expr1);
                            f(subs2, &e)
                        }
                    }
                })
            }
        }
    })
}

fn substitute_or<'r, F, X>(
        subs: Substitutions<'r>,
        a: &'r Expression<'r>,
        b: &'r Expression<'r>,
        variable: u64,
        value: bool,
        f: F) -> X
    where F : for<'r1> FnOnce(Substitutions<'r1>, &'r1 Expression<'r1>) -> X {
    substitute_inner(subs, a, variable, value, move |subs1, expr| {
        match expr {
            &Expression::True =>
                f(subs1, &TRUE),
            &Expression::False => {
                substitute_inner(subs1, b, variable, value, move |subs2, expr1| {
                    f(subs2, expr1)
                })
            },
            _ => {
                substitute_inner(subs1, b, variable, value, move |subs2, expr1| {
                    match expr1 {
                        &Expression::True => f(subs2, &TRUE),
                        &Expression::False => f(subs2, expr),
                        _ => {
                            let e = problem::or(expr, expr1);
                            f(subs2, &e)
                        }
                    }
                })
            }
        }
    })
}

fn substitute_not<'r, F, X>(
        subs: Substitutions<'r>,
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        f: F) -> X
    where F : for<'r1> FnOnce(Substitutions<'r1>, &'r1 Expression<'r1>) -> X {
    substitute_inner(subs, expr, variable, value, move |subs1, expr1| {
        match *expr1 {
            Expression::True => f(subs1, &FALSE),
            Expression::False => f(subs1, &TRUE),
            _ => {
                let e = Expression::Not(expr1);
                f(subs1, &e)
            }
        }
    })
}

fn substitute_inner<'r, F, X>(
        subs: Substitutions<'r>,
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        cb: F) -> X
    where F : for<'r1> FnOnce(Substitutions<'r1>, &'r1 Expression<'r1>) -> X {
    if !expr.has_var(variable) {
        return cb(subs, expr);
    };

    let expr_ptr = (expr as *const _) as *const ();
    match get_clone(&subs.map, &expr_ptr) {
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
            substitute_not(subs, a, variable, value, move |subs1, expr1| {
                substitute_end(subs1, expr_ptr, expr1, cb)
            })
        },
        &Expression::Or(_, ref a, ref b) => {
            substitute_or(subs, a, b, variable, value, move |subs1, expr1| {
                substitute_end(subs1, expr_ptr, expr1, cb)
            })
        },
        &Expression::And(_, ref a, ref b) => {
            substitute_and(subs, a, b, variable, value, move |subs1, expr1| {
                substitute_end(subs1, expr_ptr, expr1, cb)
            })
        }
    }
}

pub fn substitute<'r, F, X>(
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        cb: F) -> X
    where F : for<'r1> FnOnce(&'r1 Expression<'r1>) -> X {
    let subs = Substitutions {map: HashMap::new()};
    substitute_inner(subs, expr, variable, value, move |_, expr1| {
        cb(expr1)
    })
}
