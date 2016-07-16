use std::collections::HashMap;

use picorust::picosat;

use problem::Expression;

pub enum Solution {
    Sat,
    Unsat
}

fn tseytin<'r>(
            subs: &mut HashMap<*const (), i32>,
            stats: &mut Vec<Vec<i32>>,
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
                let a_v = tseytin(subs, stats, a, start_at);
                let b_v = tseytin(subs, stats, b, start_at);
                let c_v = *start_at;
                *start_at += 2;
                stats.extend(vec![vec![c_v, -a_v, -b_v], vec![-c_v, a_v], vec![-c_v, b_v]]);
                c_v
            },
            &Expression::Or(_, a, b) => {
                let a_v = tseytin(subs, stats, a, start_at);
                let b_v = tseytin(subs, stats, b, start_at);
                let c_v = *start_at;
                *start_at += 2;
                stats.extend(vec![vec![-c_v, a_v, b_v], vec![c_v, -a_v], vec![c_v, -b_v]]);
                c_v
            },
            &Expression::Not(a) => {
                -tseytin(subs, stats, a, start_at)
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

pub fn solve<'r>(expr: &'r Expression<'r>) -> Solution {
    let mut start_at = 3;
    let mut subs = HashMap::new();
    let mut stats = vec![vec![1]];
    let output = tseytin(&mut subs, &mut stats, expr, &mut start_at);
    stats.push(vec![output]);
    let mut solver = picosat::init();

    for statement in stats {
        picosat::add_arg(&mut solver, statement.as_slice());
    }

    match picosat::sat(&mut solver, -1) as isize {
        picosat::PICOSAT_SATISFIABLE => Solution::Sat,
        picosat::PICOSAT_UNSATISFIABLE => Solution::Unsat,
        _ => panic!("unknown")
    }
}
