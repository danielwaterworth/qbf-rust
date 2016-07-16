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

fn substitute_and<'r, F, X, S>(
        subs: Substitutions<'r>,
        a: &'r Expression<'r>,
        b: &'r Expression<'r>,
        variable: u64,
        value: bool,
        f: F,
        s: S) -> X
    where F : for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>, S) -> X {
    substitute_inner(subs, a, variable, value, |subs1, expr, s1| {
        match expr {
            &Expression::False =>
                f(subs1, &FALSE, s1),
            &Expression::True => {
                substitute_inner(subs1, b, variable, value, |subs2, expr1, s2| {
                    f(subs2, expr1, s2)
                }, s1)
            },
            _ => {
                substitute_inner(subs1, b, variable, value, |subs2, expr1, s2| {
                    match expr1 {
                        &Expression::False => f(subs2, &FALSE, s2),
                        &Expression::True => f(subs2, expr, s2),
                        _ => {
                            let e = problem::and(expr, expr1);
                            f(subs2, &e, s2)
                        }
                    }
                }, s1)
            }
        }
    }, s)
}

fn substitute_or<'r, F, X, S>(
        subs: Substitutions<'r>,
        a: &'r Expression<'r>,
        b: &'r Expression<'r>,
        variable: u64,
        value: bool,
        f: F,
        s: S) -> X
    where F : for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>, S) -> X {
    substitute_inner(subs, a, variable, value, |subs1, expr, s1| {
        match expr {
            &Expression::True =>
                f(subs1, &TRUE, s1),
            &Expression::False =>
                substitute_inner(subs1, b, variable, value, |subs2, expr1, s2| {
                    f(subs2, expr1, s2)
                }, s1),
            _ => {
                substitute_inner(subs1, b, variable, value, |subs2, expr1, s2| {
                    match expr1 {
                        &Expression::True => f(subs2, &TRUE, s2),
                        &Expression::False => f(subs2, expr, s2),
                        _ => {
                            let e = problem::or(expr, expr1);
                            f(subs2, &e, s2)
                        }
                    }
                }, s1)
            }
        }
    }, s)
}

fn substitute_end<'r, F, X, S>(
        mut subs: Substitutions<'r>,
        expr: &'r Expression<'r>,
        cb: F,
        s: S) -> X
    where F : for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>, S) -> X {
    let expr_ptr = (expr as *const _) as *const ();
    subs.map.insert(expr_ptr, expr);
    cb(subs, expr, s)
}

fn substitute_inner<'r, F, X, S>(
        subs: Substitutions<'r>,
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        cb: F,
        s: S) -> X
    where F : for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>, S) -> X {
    if !expr.has_var(variable) {
        return cb(subs, expr, s);
    };

    let expr_ptr = (expr as *const _) as *const ();
    match get_clone(&subs.map, &expr_ptr) {
        Some(expr1) => {
            return cb(subs, expr1, s);
        },
        None => {}
    };

    let f: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>, S) -> X = &|subs1, expr1, s1| {
        substitute_end(subs1, expr1, &cb, s1)
    };

    match expr {
        &Expression::True => substitute_end(subs, expr, &cb, s),
        &Expression::False => substitute_end(subs, expr, &cb, s),
        &Expression::Var(ref n) => {
            if *n == variable {
                if value {
                    substitute_end(subs, &TRUE, &cb, s)
                } else {
                    substitute_end(subs, &FALSE, &cb, s)
                }
            } else {
                substitute_end(subs, expr, &cb, s)
            }
        },
        &Expression::Not(ref a) => {
            let g: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>, S) -> X = &|subs1, expr1, s1| {
                match expr1 {
                    &Expression::True =>
                        substitute_end(subs1, &FALSE, &cb, s1),
                    &Expression::False =>
                        substitute_end(subs1, &TRUE, &cb, s1),
                    _ => {
                        let e = Expression::Not(expr1);
                        substitute_end(subs1, &e, &cb, s1)
                    }
                }
            };
            substitute_inner(subs, a, variable, value, g, s)
        },
        &Expression::Or(_, ref a, ref b) => {
            substitute_or(subs, a, b, variable, value, f, s)
        },
        &Expression::And(_, ref a, ref b) => {
            substitute_and(subs, a, b, variable, value, f, s)
        }
    }
}

pub fn substitute<'r, F, X, S>(
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        cb: F,
        s: S) -> X
    where F : for<'r1> Fn(&'r1 Expression<'r1>, S) -> X {
    let subs = Substitutions {map: HashMap::new()};
    substitute_inner(subs, expr, variable, value, |_, expr1, s| cb(expr1, s), s)
}
