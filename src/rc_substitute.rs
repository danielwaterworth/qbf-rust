use std::collections::HashMap;
use std::rc::Rc;

use rc_expression::Exp;

struct Substituter {
    variable: u32,
    value: bool,
    subs: HashMap<*const Exp, Rc<Exp>>,
}

impl Substituter {
    fn new(variable: u32, value: bool) -> Substituter {
        Substituter {
            variable: variable,
            value: value,
            subs: HashMap::new()
        }
    }

    fn substitute(&mut self, exp: Rc<Exp>) -> Rc<Exp> {
        let expr_ptr = &*exp as *const _;
        match self.subs.get(&expr_ptr).map(|v| v.clone()) {
            Some(v1) => v1.clone(),
            None => {
                let outcome =
                    match &*exp {
                        &Exp::And(ref a, ref b) => {
                            let a1 = self.substitute(a.clone());
                            let b1 = self.substitute(b.clone());
                            if ((&**a as *const _) == (&*a1 as *const _)) &&
                               ((&**b as *const _) == (&*b1 as *const _)) {
                                exp.clone()
                            } else {
                                Exp::and(a1, b1)
                            }
                        },
                        &Exp::Not(ref a) => {
                            let a1 = self.substitute(a.clone());
                            if (&**a as *const _) == (&*a1 as *const _) {
                                exp.clone()
                            } else {
                                Exp::not(a1)
                            }
                        },
                        &Exp::Var(n) => {
                            if self.variable == n {
                                if self.value {
                                    Rc::new(Exp::True)
                                } else {
                                    Rc::new(Exp::False)
                                }
                            } else {
                                exp.clone()
                            }
                        },
                        _ => exp.clone()
                    };
                self.subs.insert(expr_ptr, outcome.clone());
                outcome
            }
        }
    }
}

pub fn substitute(
        expr: Rc<Exp>,
        variable: u32,
        value: bool) -> Rc<Exp>
{
    Substituter::new(variable, value).substitute(expr)
}
