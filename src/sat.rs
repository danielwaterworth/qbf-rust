use std::collections::HashMap;

use picorust::picosat;

use problem::Expression;

pub enum Solution {
    Sat,
    Unsat
}

fn tseytin<'r>(
            subs: &mut HashMap<*const (), i32>,
            pico: &mut picosat::PicoSAT,
            expr: &'r Expression<'r>,
            start_at: &mut i32
        ) -> i32 {
    let expr_p = (expr as *const _) as *const ();

    match subs.get(&expr_p) {
        Some(output) => {
            return *output;
        },
        None => {}
    };

    assert!(*start_at % 2 == 1);
    let output =
        match expr {
            &Expression::And(_, a, b) => {
                let a_v = tseytin(subs, pico, a, start_at);
                let b_v = tseytin(subs, pico, b, start_at);
                let c_v = *start_at;
                *start_at += 2;
                picosat::add_arg(pico, &[c_v, -a_v, -b_v]);
                picosat::add_arg(pico, &[-c_v, a_v]);
                picosat::add_arg(pico, &[-c_v, b_v]);
                c_v
            },
            &Expression::Not(a) => {
                -tseytin(subs, pico, a, start_at)
            },
            &Expression::True => {
                1
            },
            &Expression::False => {
                -1
            },
            &Expression::Var(n) => {
                let n1 = (n + 1) * 2;
                assert!(n1 % 2 == 0);
                assert!((n1 as i32) as u64 == n1);
                n1 as i32
            }
        };
    subs.insert(expr_p, output);
    output
}

pub struct SATSolver {
    pico: picosat::PicoSAT
}

impl SATSolver {
    pub fn new<'r>(expr: &'r Expression<'r>) -> SATSolver {
        let mut start_at = 3;
        let mut subs = HashMap::new();
        let mut solver = picosat::init();
        let output = tseytin(&mut subs, &mut solver, expr, &mut start_at);
        picosat::add_arg(&mut solver, &[output]);
        SATSolver { pico: solver }
    }

    pub fn solve(&mut self) -> Solution {
        match picosat::sat(&mut self.pico, -1) as isize {
            picosat::PICOSAT_SATISFIABLE => Solution::Sat,
            picosat::PICOSAT_UNSATISFIABLE => Solution::Unsat,
            _ => panic!("unknown")
        }
    }

    pub fn set_var(&mut self, variable: u64, value: bool) {
        let n1 = (variable + 1) * 2;
        assert!(n1 % 2 == 0);
        let n2 = n1 as i32;
        assert!(n2 as u64 == n1);

        picosat::push(&mut self.pico);
        if value {
            picosat::add_arg(&mut self.pico, &[n2]);
        } else {
            picosat::add_arg(&mut self.pico, &[-n2]);
        }
    }

    pub fn unset(&mut self) {
        picosat::pop(&mut self.pico);
    }
}
