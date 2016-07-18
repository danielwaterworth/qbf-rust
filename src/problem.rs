use vars::Vars;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Quantifier {
    Exists,
    ForAll
}

pub fn opposite_quantifier(q: Quantifier) -> Quantifier {
    match q {
        Quantifier::Exists => Quantifier::ForAll,
        Quantifier::ForAll => Quantifier::Exists
    }
}

#[derive(Debug)]
pub enum Expression<'r> {
    And(Vars, &'r Expression<'r>, &'r Expression<'r>),
    Not(&'r Expression<'r>),
    Var(u64),
    True,
    False
}

impl<'r> PartialEq for Expression<'r> {
    fn eq(&self, other: &Expression<'r>) -> bool {
        match (self, other) {
            (&Expression::True, &Expression::True) => true,
            (&Expression::False, &Expression::False) => true,
            (&Expression::Var(n), &Expression::Var(m)) => n == m,
            (&Expression::Not(a), &Expression::Not(b)) => a == b,
            (&Expression::And(_, a, b), &Expression::And(_, p, q)) => a == p && b == q,
            (_, _) => false
        }
    }
}

impl<'r> Expression<'r> {
    pub fn has_var(&self, var: u64) -> bool {
        match self {
            &Expression::And(ref v, _, _) => v.get(var),
            &Expression::Not(ref v) => v.has_var(var),
            &Expression::Var(v) => v == var,
            _ => false
        }
    }

    fn with_variables<F, X>(&self, f: F) -> X
        where F: for<'r1> FnOnce(&'r1 Vars) -> X {
        match self {
            &Expression::And(ref v, _, _) => f(v),
            &Expression::Not(ref v) => v.with_variables(f),
            &Expression::Var(v) => {
                let mut output = Vars::new();
                output.add(v);
                f(&output)
            },
            _ => f(&Vars::new())
        }
    }

    fn variables(&self) -> Vars {
        self.with_variables(|v| v.clone())
    }
}

pub fn and<'a, F, X>(
        a: &'a Expression<'a>,
        b: &'a Expression<'a>,
        f: F) -> X
    where F: for<'b> FnOnce(&'b Expression<'b>) -> X
{
    match (a, b) {
        (&Expression::False, _) => f(&FALSE),
        (_, &Expression::False) => f(&FALSE),
        (&Expression::True, _) => f(b),
        (_, &Expression::True) => f(a),
        _ => {
            let mut v_a = a.variables();
            let mut v_b = b.variables();
            v_a.union(&mut v_b);
            let e = Expression::And(v_a, a, b);
            f(&e)
        }
    }
}

pub fn not<'r, F, X>(
        a: &'r Expression<'r>,
        f: F) -> X
    where F: for<'b> FnOnce(&'b Expression<'b>) -> X
{
    match a {
        &Expression::True => f(&FALSE),
        &Expression::False => f(&TRUE),
        &Expression::Not(ref e) => f(e),
        _ => {
            let e = Expression::Not(a);
            f(&e)
        }
    }
}

pub fn or<'a, F, X>(
        a: &'a Expression<'a>,
        b: &'a Expression<'a>,
        f: F) -> X
    where F: for<'b> FnOnce(&'b Expression<'b>) -> X
{
    not(a, |a_| {
        not(b, |b_| {
            and(a_, b_, |e| {
                not(e, f)
            })
        })
    })
}

pub static TRUE: Expression<'static> = Expression::True;
pub static FALSE: Expression<'static> = Expression::False;

#[derive(Debug)]
pub struct QBF<'r> {
    pub first_quantifier: Quantifier,
    pub quantifier_blocks: &'r [u64],
    pub expr: &'r Expression<'r>
}

#[derive(Debug)]
pub enum Solution {
    Sat,
    Unsat
}
