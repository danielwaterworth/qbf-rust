use std::collections::HashMap;

use std::rc::Rc;

use rc_expression::Expression as RExp;

#[derive(Debug)]
pub enum Expression {
    And(Vec<Rc<Expression>>),
    Not(Rc<Expression>),
    Var(u32),
    True,
    False
}

pub fn and(a: Rc<Expression>, b: Rc<Expression>) -> Rc<Expression> {
    let ref a1 = *a;
    let ref b1 = *b;
    match (a1, b1) {
        (&Expression::And(ref x), &Expression::And(ref y)) => {
            let mut z = x.clone();
            z.extend(y.iter().cloned());
            Rc::new(Expression::And(z))
        },
        (&Expression::And(ref x), _) => {
            let mut z = x.clone();
            z.push(b.clone());
            Rc::new(Expression::And(z))
        },
        (_, &Expression::And(ref x)) => {
            let mut z = x.clone();
            z.push(a.clone());
            Rc::new(Expression::And(z))
        },
        _ => {
            Rc::new(Expression::And(vec![a.clone(), b.clone()]))
        }
    }
}

pub fn not(x: Rc<Expression>) -> Rc<Expression> {
    match *x {
        Expression::Not(ref y) => y.clone(),
        Expression::True => Rc::new(Expression::False),
        Expression::False => Rc::new(Expression::True),
        _ => {
            Rc::new(Expression::Not(x.clone()))
        }
    }
}

pub fn or(a: Rc<Expression>, b: Rc<Expression>) -> Rc<Expression> {
    not(and(not(a), not(b)))
}

struct NExpBuilder {
    replacements: HashMap<*const (), Rc<Expression>>
}

impl NExpBuilder {
    fn new() -> NExpBuilder {
        NExpBuilder { replacements: HashMap::new() }
    }

    fn build(&mut self, exp: Rc<RExp>) -> Rc<Expression> {
        let expr_ptr = &*exp as *const _ as *const ();
        match self.replacements.get(&expr_ptr).map(|v| v.clone()) {
            Some(e) => e.clone(),
            None => {
                let outcome =
                    match *exp {
                        RExp::And(ref a, ref b) => {
                            let a1 = self.build(a.clone());
                            let b1 = self.build(b.clone());
                            and(a1, b1)
                        },
                        RExp::Not(ref x) => {
                            let x1 = self.build(x.clone());
                            not(x1)
                        },
                        RExp::True => {
                            Rc::new(Expression::True)
                        },
                        RExp::False => {
                            Rc::new(Expression::False)
                        },
                        RExp::Var(n) => {
                            Rc::new(Expression::Var(n))
                        }
                    };
                self.replacements.insert(expr_ptr, outcome.clone());
                outcome
            }
        }
    }
}

struct RExpBuilder {
    replacements: HashMap<*const (), Rc<RExp>>
}

impl RExpBuilder {
    fn new() -> RExpBuilder {
        RExpBuilder { replacements: HashMap::new() }
    }

    fn build_and(&mut self, exps: &[Rc<Expression>]) -> Rc<RExp> {
        assert!(exps.len() != 0);
        if exps.len() == 1 {
            self.build(exps[0].clone())
        } else {
            let m = exps.len() / 2;
            let a = self.build_and(&exps[0..m]);
            let b = self.build_and(&exps[m..exps.len()]);
            Rc::new(RExp::And(a, b))
        }
    }

    fn build(&mut self, exp: Rc<Expression>) -> Rc<RExp> {
        let expr_ptr = &*exp as *const _ as *const ();
        match self.replacements.get(&expr_ptr).map(|v| v.clone()) {
            Some(e) => e.clone(),
            None => {
                let outcome =
                    match *exp {
                        Expression::And(ref v) => {
                            self.build_and(v)
                        },
                        Expression::Not(ref x) => {
                            let x1 = self.build(x.clone());
                            Rc::new(RExp::Not(x1))
                        },
                        Expression::True => {
                            Rc::new(RExp::True)
                        },
                        Expression::False => {
                            Rc::new(RExp::False)
                        },
                        Expression::Var(n) => {
                            Rc::new(RExp::Var(n))
                        }
                    };
                self.replacements.insert(expr_ptr, outcome.clone());
                outcome
            }
        }
    }
}

pub fn nexp_to_rexp(exp: Rc<Expression>) -> Rc<RExp> {
    RExpBuilder::new().build(exp)
}

impl Expression {
    pub fn from_rexp(exp: Rc<RExp>) -> Rc<Expression> {
        NExpBuilder::new().build(exp)
    }
}
