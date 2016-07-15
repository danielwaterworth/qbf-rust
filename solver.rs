use std::collections::HashMap;
use std::hash::Hash;

use problem::Quantifier;
use problem::Expression;
use problem::QBF;
use problem::TRUE;
use problem::FALSE;
use problem::opposite_quantifier;

#[derive(Debug)]
pub enum Solution {
    Sat,
    Unsat
}

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

fn substitute<'r, F, X>(
        subs: Substitutions<'r>,
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        cb: F) -> X
    where F : for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X {
    let expr_ptr = (expr as *const _) as *const ();
    match get_clone(&subs.map, &expr_ptr) {
        Some(expr1) => {
            return cb(subs, expr1);
        },
        None => {}
    };

    let f: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|mut subs1, expr1| {
        subs1.map.insert(expr_ptr, expr1);
        cb(subs1, expr1)
    };

    match *expr {
        Expression::True => f(subs, expr),
        Expression::False => f(subs, expr),
        Expression::Var(n) => {
            if n == variable {
                if value {
                    f(subs, &TRUE)
                } else {
                    f(subs, &FALSE)
                }
            } else {
                f(subs, expr)
            }
        },
        Expression::Not(a) => {
            let g: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs1, expr1| {
                match *expr1 {
                    Expression::True => f(subs1, &FALSE),
                    Expression::False => f(subs1, &TRUE),
                    _ => {
                        let e = Expression::Not(expr1);
                        f(subs1, &e)
                    }
                }
            };
            substitute(subs, a, variable, value, g)
        },
        Expression::Or(a, b) => {
            let g: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs1, expr| {
                match *expr {
                    Expression::True => f(subs1, &TRUE),
                    Expression::False => {
                        let h: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs2, expr1| f(subs2, expr1);
                        substitute(subs1, b, variable, value, h)
                    },
                    _ => {
                        let h: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs2, expr1| {
                            match *expr1 {
                                Expression::True => f(subs2, &TRUE),
                                Expression::False => f(subs2, expr1),
                                _ => {
                                    let e = Expression::Or(expr, expr1);
                                    f(subs2, &e)
                                }
                            }
                        };
                        substitute(subs1, b, variable, value, h)
                    }
                }
            };
            substitute(subs, a, variable, value, g)
        },
        Expression::And(a, b) => {
            let g: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs1, expr| {
                match *expr {
                    Expression::False => f(subs1, &FALSE),
                    Expression::True => {
                        let h: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs2, expr1| f(subs2, expr1);
                        substitute(subs1, b, variable, value, h)
                    },
                    _ => {
                        let h: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs2, expr1| {
                            match *expr1 {
                                Expression::False => f(subs2, &FALSE),
                                Expression::True => f(subs2, expr1),
                                _ => {
                                    let e = Expression::And(expr, expr1);
                                    f(subs2, &e)
                                }
                            }
                        };
                        substitute(subs1, b, variable, value, h)
                    }
                }
            };
            substitute(subs, a, variable, value, g)
        }
    }
}

fn quantifier_blocks(quantifiers: &[Quantifier]) -> (Quantifier, Vec<u64>) {
    let first_quantifier = quantifiers[0].clone();
    let mut output = vec![];

    let mut current_quantifier = first_quantifier.clone();
    let mut n = 1;

    for quantifier in &quantifiers[1..] {
        if quantifier.clone() == current_quantifier {
            n += 1;
        } else {
            output.push(n);
            n = 1;
            current_quantifier = quantifier.clone();
        }
    }
    output.push(n);

    return (first_quantifier, output);
}

fn solve_inner_with<'r>(
            current_quantifier: Quantifier,
            current_block: u64,
            blocks: &[u64],
            start_at: u64,
            expr: &'r Expression<'r>,
            value: bool
        ) -> Solution {
    let solve1: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> Solution = &|_, expr1| {
        solve_inner(
            current_quantifier,
            current_block - 1,
            blocks,
            start_at + 1,
            expr1
        )
    };
    let subs = Substitutions {map: HashMap::new()};
    substitute(subs, expr, start_at, value, solve1)
}

fn solve_inner<'r>(
            mut current_quantifier: Quantifier,
            mut current_block: u64,
            mut blocks: &[u64],
            start_at: u64,
            expr: &'r Expression<'r>
        ) -> Solution {
    if current_block == 0 {
        if blocks.len() == 0 {
            match *expr {
                Expression::True => return Solution::Sat,
                Expression::False => return Solution::Unsat,
                _ => panic!("free variable")
            }
        } else {
            current_quantifier = opposite_quantifier(current_quantifier);
            current_block = blocks[0];
            blocks = &blocks[1..];
        }
    };

    match current_quantifier {
        Quantifier::ForAll => {
            match solve_inner_with(current_quantifier, current_block, blocks, start_at, expr, false) {
                Solution::Sat => {
                    solve_inner_with(current_quantifier, current_block, blocks, start_at, expr, true)
                },
                Solution::Unsat => Solution::Unsat
            }
        },
        Quantifier::Exists => {
            match solve_inner_with(current_quantifier, current_block, blocks, start_at, expr, false) {
                Solution::Sat => Solution::Sat,
                Solution::Unsat => {
                    solve_inner_with(current_quantifier, current_block, blocks, start_at, expr, true)
                }
            }
        }
    }
}

pub fn solve<'r>(problem: &'r QBF<'r>) -> Solution {
    let (first_quantifier, blocks) = quantifier_blocks(&problem.quantifiers);
    solve_inner(first_quantifier, blocks[0], &blocks[1..], 0, problem.expr)
}
