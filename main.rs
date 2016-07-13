enum Quantifier {
    Exists,
    ForAll
}

enum Expression<'r> {
    And(&'r Expression<'r>, &'r Expression<'r>),
    Or(&'r Expression<'r>, &'r Expression<'r>),
    Not(&'r Expression<'r>),
    Var(u32),
    True,
    False
}

struct QBF<'r> {
    start_at: u32,
    quantifiers: &'r [Quantifier],
    expr: &'r Expression<'r>
}

enum Solution {
    Sat,
    Unsat
}

static TRUE: Expression<'static> = Expression::True;
static FALSE: Expression<'static> = Expression::False;

fn substitute<'r, F, X>(expr: &'r Expression<'r>, variable: u32, value: bool, f: F) -> X
    where F : for<'r1> Fn(&'r1 Expression<'r1>) -> X {
    match *expr {
        Expression::True => f(expr),
        Expression::False => f(expr),
        Expression::Var(n) => {
            if n == variable {
                if value {
                    f(&TRUE)
                } else {
                    f(&FALSE)
                }
            } else {
                f(expr)
            }
        },
        Expression::Not(a) => {
            let g: &for<'r1> Fn(&'r1 Expression<'r1>) -> X = &|expr| {
                match *expr {
                    Expression::True => f(&FALSE),
                    Expression::False => f(&TRUE),
                    _ => {
                        let e = Expression::Not(expr);
                        f(&e)
                    }
                }
            };
            substitute(a, variable, value, g)
        },
        Expression::Or(a, b) => {
            let g: &for<'r1> Fn(&'r1 Expression<'r1>) -> X = &|expr| {
                match *expr {
                    Expression::True => f(&TRUE),
                    Expression::False => {
                        let h: &for<'r1> Fn(&'r1 Expression<'r1>) -> X = &|expr1| f(expr1);
                        substitute(b, variable, value, h)
                    },
                    _ => {
                        let h: &for<'r1> Fn(&'r1 Expression<'r1>) -> X = &|expr1| {
                            match *expr1 {
                                Expression::True => f(&TRUE),
                                Expression::False => f(expr1),
                                _ => {
                                    let e = Expression::Or(expr, expr1);
                                    f(&e)
                                }
                            }
                        };
                        substitute(b, variable, value, h)
                    }
                }
            };
            substitute(a, variable, value, g)
        },
        Expression::And(a, b) => {
            let g: &for<'r1> Fn(&'r1 Expression<'r1>) -> X = &|expr| {
                match *expr {
                    Expression::False => f(&FALSE),
                    Expression::True => {
                        let h: &for<'r1> Fn(&'r1 Expression<'r1>) -> X = &|expr1| f(expr1);
                        substitute(b, variable, value, h)
                    },
                    _ => {
                        let h: &for<'r1> Fn(&'r1 Expression<'r1>) -> X = &|expr1| {
                            match *expr1 {
                                Expression::False => f(&FALSE),
                                Expression::True => f(expr1),
                                _ => {
                                    let e = Expression::And(expr, expr1);
                                    f(&e)
                                }
                            }
                        };
                        substitute(b, variable, value, h)
                    }
                }
            };
            substitute(a, variable, value, g)
        }
    }
}

fn solve_with<'r>(problem : &'r QBF<'r>, v: bool) -> Solution {
    let solve1: &for<'r1> Fn(&'r1 Expression<'r1>) -> Solution = &|expr|
        solve(
            &QBF {
                start_at: problem.start_at + 1,
                quantifiers: &problem.quantifiers[1..],
                expr: expr
            });
    substitute(&problem.expr, problem.start_at, v, solve1)
}

fn solve<'r>(problem: &'r QBF<'r>) -> Solution {
    if problem.quantifiers.is_empty() {
        match *problem.expr {
            Expression::True => Solution::Sat,
            Expression::False => Solution::Unsat,
            _ => panic!("free variable")
        }
    } else {
        match problem.quantifiers[0] {
            Quantifier::ForAll => {
                match solve_with(problem, false) {
                    Solution::Sat => solve_with(problem, true),
                    Solution::Unsat => Solution::Unsat
                }
            },
            Quantifier::Exists => {
                match solve_with(problem, false) {
                    Solution::Sat => Solution::Sat,
                    Solution::Unsat => solve_with(problem, true)
                }
            }
        }
    }
}

fn main() {
    let n = Expression::Var(0);
    let m = Expression::Var(1);
    let e = Expression::And(&n, &m);
    let e1 = Expression::Not(&e);
    let quantifiers = [Quantifier::Exists, Quantifier::ForAll];
    let q = QBF {
        start_at: 0,
        quantifiers: &quantifiers,
        expr: &e1
    };
    match solve(&q) {
        Solution::Sat => println!("sat"),
        Solution::Unsat => println!("unsat")
    }
}
