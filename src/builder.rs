use std::collections::HashMap;

use problem;
use problem::Expression as QExp;

pub struct Builder<'r> {
    variables: HashMap<String, &'r QExp<'r>>,
}

impl<'r> Builder<'r> {
    pub fn new(variables: HashMap<String, &'r QExp<'r>>) -> Builder<'r> {
        Builder{
            variables: variables,
        }
    }

    pub fn set(&mut self, name: String, exp: &'r QExp<'r>) {
        self.variables.insert(name, exp);
    }

    pub fn get(&self, name: &String) -> &'r QExp<'r> {
        self.variables.get(name).unwrap()
    }

    pub fn var<X>(
        self,
        v: u64,
        f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
    {
        let e = QExp::Var(v);
        f(self, &e)
    }

    pub fn not<X>(
        self,
        a: &'r QExp<'r>,
        f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
    {
        let e = problem::not(a);
        f(self, &e)
    }

    pub fn and<X>(
        self,
        a: &'r QExp<'r>,
        b: &'r QExp<'r>,
        f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
    {
        let e = problem::and(a, b);
        f(self, &e)
    }

    pub fn or<X>(
        self,
        a: &'r QExp<'r>,
        b: &'r QExp<'r>,
        f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
    {
        self.not(a, &mut |builder1, a_| {
            builder1.not(b, &mut |builder2, b_| {
                builder2.and(a_, b_, &mut |builder3, e| {
                    builder3.not(e, f)
                })
            })
        })
    }
}
