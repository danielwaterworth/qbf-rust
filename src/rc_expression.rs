use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;

use problem;
use problem::Expression as QExp;

#[derive(Debug)]
pub enum Exp {
    And(Rc<Exp>, Rc<Exp>),
    Not(Rc<Exp>),
    Var(u32),
    True,
    False
}

fn construct_inner<'a>(
        replacements: &mut HashMap<*const (), Rc<Exp>>,
        exp: &'a QExp<'a>) -> Rc<Exp>
{
    let expr_ptr = exp as *const _ as *const ();
    match replacements.get(&expr_ptr).map(|v| v.clone()) {
        Some(e) => e,
        None => {
            let outcome =
                match exp {
                    &QExp::And(_, a, b) => {
                        let a1 = construct_inner(replacements, a);
                        let b1 = construct_inner(replacements, b);
                        Exp::and(a1, b1)
                    },
                    &QExp::Not(x) => {
                        let x1 = construct_inner(replacements, x);
                        Exp::not(x1)
                    },
                    &QExp::Var(n) => {
                        Rc::new(Exp::Var(n))
                    },
                    &QExp::True => {
                        Rc::new(Exp::True)
                    },
                    &QExp::False => {
                        Rc::new(Exp::False)
                    }
                };
            replacements.insert(expr_ptr, outcome.clone());
            outcome
        }
    }
}

pub fn construct<'a>(exp: &'a QExp<'a>) -> Rc<Exp> {
    construct_inner(&mut HashMap::new(), exp)
}

fn with_inner_end<'a, F, X>(
    mut replacements: HashMap<*const (), &'a QExp<'a>>,
    expr_ptr: *const (),
    exp: &'a QExp<'a>,
    f: &mut F) -> X
    where F : for<'b> FnMut(HashMap<*const (), &'b QExp<'b>>, &'b QExp<'b>) -> X
{
    replacements.insert(expr_ptr, &exp);
    f(replacements, exp)
}

fn with_inner<'a, X>(
    replacements: HashMap<*const (), &'a QExp<'a>>,
    exp: Rc<Exp>,
    mut f: &mut (for<'b> FnMut(HashMap<*const (), &'b QExp<'b>>, &'b QExp<'b>) -> X + 'a)) -> X
{
    let expr_ptr = &*exp as *const _ as *const ();
    match replacements.get(&expr_ptr).map(|v| v.clone()) {
        Some(exp1) => f(replacements, exp1),
        None => {
            match *exp {
                Exp::And(ref a, ref b) => {
                    with_inner(replacements, a.clone(), &mut |replacements1, a1| {
                        with_inner(replacements1, b.clone(), &mut |replacements2, b1| {
                            problem::and(a1, b1, |e| {
                                with_inner_end(replacements2, expr_ptr, e, &mut f)
                            })
                        })
                    })
                },
                Exp::Not(ref x) => {
                    with_inner(replacements, x.clone(), &mut |replacements1, x1| {
                        problem::not(x1, |e| {
                            with_inner_end(replacements1, expr_ptr, e, &mut f)
                        })
                    })
                },
                Exp::Var(var) => {
                    with_inner_end(replacements, expr_ptr, &QExp::Var(var), &mut f)
                },
                Exp::True => {
                    with_inner_end(replacements, expr_ptr, &problem::TRUE, &mut f)
                },
                Exp::False => {
                    with_inner_end(replacements, expr_ptr, &problem::FALSE, &mut f)
                }
            }
        }
    }
}

pub fn with<X>(
    exp: Rc<Exp>,
    f: &mut (for<'b> FnMut(&'b QExp<'b>) -> X)) -> X
{
    with_inner(HashMap::new(), exp, &mut |_, e| f(e))
}

fn same_thing<X>(a: &X, b: &X) -> bool {
    (a as *const _) == (b as *const _)
}

fn implied(exp: Rc<Exp>) -> (HashSet<*const Exp>, HashSet<*const Exp>) {
    let mut trues = HashSet::new();
    let mut falses = HashSet::new();
    let mut to_visit = vec![exp];

    while let Some(x) = to_visit.pop() {
        let expr_ptr = &*x as *const _;
        trues.insert(expr_ptr);
        match &*x {
            &Exp::And(ref p, ref q) => {
                to_visit.push(p.clone());
                to_visit.push(q.clone());
            },
            &Exp::Not(ref u) => {
                falses.insert(&**u as *const _);
            },
            _ => {}
        }
    }

    (trues, falses)
}

impl Exp {
    pub fn not(a: Rc<Exp>) -> Rc<Exp> {
        match &*a {
            &Exp::True => Rc::new(Exp::False),
            &Exp::False => Rc::new(Exp::True),
            &Exp::Not(ref e) => e.clone(),
            _ => Rc::new(Exp::Not(a.clone()))
        }
    }

    pub fn and(a: Rc<Exp>, b: Rc<Exp>) -> Rc<Exp> {
        let ref a1 = *a.clone();
        let ref b1 = *b.clone();
        match (a1, b1) {
            (&Exp::False, _) => return a.clone(),
            (_, &Exp::False) => return b.clone(),
            (&Exp::True, _) => return b.clone(),
            (_, &Exp::True) => return a.clone(),
            (&Exp::And(ref p, ref q), _) if same_thing(&**p, b1) || same_thing(&**q, b1) => return a.clone(),
            (_, &Exp::And(ref p, ref q)) if same_thing(&**p, a1) || same_thing(&**q, a1) => return b.clone(),
            (_, &Exp::Not(ref v)) => {
                let ref v1 = *v.clone();
                match v1 {
                    &Exp::And(ref q, ref p) => {
                        if same_thing(&**q, &*a) {
                            return Exp::and(a.clone(), Exp::not(p.clone()));
                        } else if same_thing(&**p, &*a) {
                            return Exp::and(a.clone(), Exp::not(q.clone()));
                        }
                    },
                    _ => {}
                }
            },
            (&Exp::Not(ref u), _) => {
                let ref u1 = *u.clone();
                match u1 {
                    &Exp::And(ref q, ref p) => {
                        if same_thing(&**q, &*b) {
                            return Exp::and(b.clone(), Exp::not(p.clone()));
                        } else if same_thing(&**p, &*b) {
                            return Exp::and(b.clone(), Exp::not(q.clone()));
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        let (a_implied_true, a_implied_false) = implied(a.clone());
        let (b_implied_true, b_implied_false) = implied(b.clone());

        if a_implied_true.intersection(&b_implied_false).next().is_some() ||
           a_implied_false.intersection(&b_implied_true).next().is_some() {
            Rc::new(Exp::False)
        } else {
            Rc::new(Exp::And(a, b))
        }
    }

    pub fn size(&self) -> usize {
        let mut visited = HashSet::new();
        let mut size = 0;

        let mut to_visit = vec![self];
        while let Some(node) = to_visit.pop() {
            let expr_ptr = node as (*const _);
            if !visited.contains(&expr_ptr) {
                visited.insert(expr_ptr);
                size += 1;
                match node {
                    &Exp::And(ref a, ref b) => {
                        to_visit.push(&*a);
                        to_visit.push(&*b);
                    },
                    &Exp::Not(ref a) => {
                        to_visit.push(&*a);
                    }
                    _ => {}
                }
            }
        }

        size
    }
}
