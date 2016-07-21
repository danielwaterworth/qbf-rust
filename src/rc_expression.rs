use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;

use problem;
use problem::Expression as QExp;

#[derive(Debug)]
pub enum Expression {
    And(Rc<Expression>, Rc<Expression>),
    Not(Rc<Expression>),
    Var(u32),
    True,
    False
}

fn construct_inner<'a>(
        replacements: &mut HashMap<*const (), Rc<Expression>>,
        exp: &'a QExp<'a>) -> Rc<Expression>
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
                        Expression::and(a1, b1)
                    },
                    &QExp::Not(x) => {
                        let x1 = construct_inner(replacements, x);
                        Expression::not(x1)
                    },
                    &QExp::Var(n) => {
                        Rc::new(Expression::Var(n))
                    },
                    &QExp::True => {
                        Rc::new(Expression::True)
                    },
                    &QExp::False => {
                        Rc::new(Expression::False)
                    }
                };
            replacements.insert(expr_ptr, outcome.clone());
            outcome
        }
    }
}

pub fn construct<'a>(exp: &'a QExp<'a>) -> Rc<Expression> {
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
    exp: Rc<Expression>,
    mut f: &mut (for<'b> FnMut(HashMap<*const (), &'b QExp<'b>>, &'b QExp<'b>) -> X + 'a)) -> X
{
    let expr_ptr = &*exp as *const _ as *const ();
    match replacements.get(&expr_ptr).map(|v| v.clone()) {
        Some(exp1) => f(replacements, exp1),
        None => {
            match *exp {
                Expression::And(ref a, ref b) => {
                    with_inner(replacements, a.clone(), &mut |replacements1, a1| {
                        with_inner(replacements1, b.clone(), &mut |replacements2, b1| {
                            problem::and(a1, b1, |e| {
                                with_inner_end(replacements2, expr_ptr, e, &mut f)
                            })
                        })
                    })
                },
                Expression::Not(ref x) => {
                    with_inner(replacements, x.clone(), &mut |replacements1, x1| {
                        problem::not(x1, |e| {
                            with_inner_end(replacements1, expr_ptr, e, &mut f)
                        })
                    })
                },
                Expression::Var(var) => {
                    with_inner_end(replacements, expr_ptr, &QExp::Var(var), &mut f)
                },
                Expression::True => {
                    with_inner_end(replacements, expr_ptr, &problem::TRUE, &mut f)
                },
                Expression::False => {
                    with_inner_end(replacements, expr_ptr, &problem::FALSE, &mut f)
                }
            }
        }
    }
}

pub fn with<X>(
    exp: Rc<Expression>,
    f: &mut (for<'b> FnMut(&'b QExp<'b>) -> X)) -> X
{
    with_inner(HashMap::new(), exp, &mut |_, e| f(e))
}

fn same_thing<X>(a: &X, b: &X) -> bool {
    (a as *const _) == (b as *const _)
}

impl Expression {
    pub fn not(a: Rc<Expression>) -> Rc<Expression> {
        match &*a {
            &Expression::True => Rc::new(Expression::False),
            &Expression::False => Rc::new(Expression::True),
            &Expression::Not(ref e) => e.clone(),
            _ => Rc::new(Expression::Not(a.clone()))
        }
    }

    pub fn and(a: Rc<Expression>, b: Rc<Expression>) -> Rc<Expression> {
        let ref a1 = *a;
        let ref b1 = *b;
        match (a1, b1) {
            (&Expression::False, _) => return a.clone(),
            (_, &Expression::False) => return b.clone(),
            (&Expression::True, _) => return b.clone(),
            (_, &Expression::True) => return a.clone(),
            (&Expression::And(ref p, ref q), _) if same_thing(&**p, b1) || same_thing(&**q, b1) => return a.clone(),
            (_, &Expression::And(ref p, ref q)) if same_thing(&**p, a1) || same_thing(&**q, a1) => return b.clone(),
            _ => {}
        }

        Rc::new(Expression::And(a.clone(), b.clone()))
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
                    &Expression::And(ref a, ref b) => {
                        to_visit.push(&*a);
                        to_visit.push(&*b);
                    },
                    &Expression::Not(ref a) => {
                        to_visit.push(&*a);
                    }
                    _ => {}
                }
            }
        }

        size
    }
}
