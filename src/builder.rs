use std::rc::Rc;

use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;

use rc_expression::Exp as Exp;
use rc_expression::Builder as RBuilder;

pub struct Builder {
    ands: HashMap<(*const (), *const ()), Rc<Exp>>,
    nots: HashMap<*const (), Rc<Exp>>,

    // this forces everything that is referenced to stick around
    uses: HashMap<*const (), Rc<Exp>>,

    rbuilder: RBuilder
}

impl Builder {
    pub fn new() -> Builder {
        Builder{
            ands: HashMap::new(),
            nots: HashMap::new(),
            uses: HashMap::new(),
            rbuilder: RBuilder::new()
        }
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
        self.uses.insert(expr_ptr.clone(), a.clone());
        match self.nots.get(&expr_ptr).map(|v| v.clone()) {
            Some(e) => e,
            None => {
                let e = self.rbuilder.not(a);
                self.nots.insert(expr_ptr, e.clone());
                e
            }
        }
    }

    pub fn and(&mut self, a: Rc<Exp>, b: Rc<Exp>) -> Rc<Exp> {
        let a_ptr = &*a as (*const _) as (*const ());
        let b_ptr = &*b as (*const _) as (*const ());
        let k = (min(a_ptr, b_ptr), max(a_ptr, b_ptr));
        self.uses.insert(a_ptr.clone(), a.clone());
        self.uses.insert(b_ptr.clone(), b.clone());

        match self.ands.get(&k).map(|v| v.clone()) {
            Some(e) => {
                e
            },
            None => {
                let e = self.rbuilder.and(a, b);
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
