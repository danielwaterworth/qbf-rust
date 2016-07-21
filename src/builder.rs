use std::rc::Rc;

use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;

use rc_expression::Expression as Exp;

pub struct Builder {
    variables: HashMap<String, Rc<Exp>>,
    ands: HashMap<(*const (), *const ()), Rc<Exp>>,
    nots: HashMap<*const (), Rc<Exp>>
}

impl Builder {
    pub fn new(variables: HashMap<String, Rc<Exp>>) -> Builder {
        Builder{
            variables: variables,
            ands: HashMap::new(),
            nots: HashMap::new()
        }
    }

    pub fn set(&mut self, name: String, exp: Rc<Exp>) {
        self.variables.insert(name, exp);
    }

    pub fn get(&self, name: &String) -> Rc<Exp> {
        self.variables.get(name).unwrap().clone()
    }

    pub fn var(&self, v: u32) -> Rc<Exp> {
        Rc::new(Exp::Var(v))
    }

    pub fn true_(&self) -> Rc<Exp> {
        Rc::new(Exp::True)
    }

    pub fn false_(&self) -> Rc<Exp> {
        Rc::new(Exp::True)
    }

    pub fn not(&mut self, a: Rc<Exp>) -> Rc<Exp> {
        let expr_ptr = &*a as (*const _) as (*const ());
        match self.nots.get(&expr_ptr).map(|v| v.clone()) {
            Some(e) => e,
            None => {
                let e = Rc::new(Exp::not(a));
                self.nots.insert(expr_ptr, e.clone());
                e
            }
        }
    }

    pub fn and(&mut self, a: Rc<Exp>, b: Rc<Exp>) -> Rc<Exp> {
        let a_ptr = &*a as (*const _) as (*const ());
        let b_ptr = &*b as (*const _) as (*const ());
        let k = (min(a_ptr, b_ptr), max(a_ptr, b_ptr));

        match self.ands.get(&k).map(|v| v.clone()) {
            Some(e) => {
                e
            },
            None => {
                let e = Rc::new(Exp::and(a, b));
                self.ands.insert(k, e.clone());
                e
            }
        }
    }

    pub fn or(&mut self, a: Rc<Exp>, b: Rc<Exp>) -> Rc<Exp> {
        let a_ = self.not(a);
        let b_ = self.not(b);
        let e = self.and(a_, b_);
        self.not(e)
    }
}
