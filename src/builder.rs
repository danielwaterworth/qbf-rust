use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;

use problem;
use problem::Expression as QExp;

pub struct Builder<'r> {
    variables: HashMap<String, &'r QExp<'r>>,
    ands: HashMap<(*const (), *const ()), &'r QExp<'r>>,
    nots: HashMap<*const (), &'r QExp<'r>>
}

impl<'r> Builder<'r> {
    pub fn new(variables: HashMap<String, &'r QExp<'r>>) -> Builder<'r> {
        Builder{
            variables: variables,
            ands: HashMap::new(),
            nots: HashMap::new()
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
        v: u32,
        f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
    {
        let e = QExp::Var(v);
        f(self, &e)
    }

    fn insert_not<X>(
            mut self,
            expr_ptr: *const (),
            e: &'r QExp<'r>,
            f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
    {
        self.nots.insert(expr_ptr, e);
        f(self, &e)
    }

    pub fn not<X>(
        self,
        a: &'r QExp<'r>,
        f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
    {
        let expr_ptr = a as (*const _) as (*const ());
        match self.nots.get(&expr_ptr).map(|v| v.clone()) {
            Some(e) => f(self, e),
            None => {
                problem::not(a, |e| {
                    self.insert_not(expr_ptr, &e, f)
                })
            }
        }
    }

    fn insert_and<X>(
            mut self,
            k: (*const (), *const ()),
            e: &'r QExp<'r>,
            f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
    {
        self.ands.insert(k, e);
        f(self, e)
    }

    pub fn and<X>(
        self,
        a: &'r QExp<'r>,
        b: &'r QExp<'r>,
        f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
    {
        let a_ptr = a as (*const _) as (*const ());
        let b_ptr = b as (*const _) as (*const ());
        let k = (min(a_ptr, b_ptr), max(a_ptr, b_ptr));

        match self.ands.get(&k).map(|v| v.clone()) {
            Some(e) => {
                f(self, e)
            },
            None => {
                problem::and(a, b, move |e| {
                    self.insert_and(k, e, f)
                })
            }
        }
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
