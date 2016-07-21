use std::rc::Rc;

use problem;
use problem::Quantifier;
use problem::Solution;
use problem::opposite_quantifier;

use rc_expression::Exp as RExp;
use rc_expression::construct;
use rc_expression::with;
use rc_expression::QBF;

use substitute::substitute;

fn expand(quantifier: Quantifier, var: u32, exp: Rc<RExp>) -> Rc<RExp> {
    with(exp, &mut |exp1| {
        substitute(exp1, var, false, |false_expr| {
            substitute(exp1, var, true, |true_expr| {
                match quantifier {
                    Quantifier::ForAll => {
                        problem::and(false_expr, true_expr, |expr| {
                            construct(expr)
                        })
                    },
                    Quantifier::Exists => {
                        problem::or(false_expr, true_expr, |expr| {
                            construct(expr)
                        })
                    }
                }
            })
        })
    })
}

pub fn solve<'r>(problem: QBF) -> Solution {
    let n_variables: u32 = problem.quantifier_blocks.iter().sum();
    let mut expr = problem.expr;

    let mut current_quantifier = problem.last_quantifier;
    let mut var = n_variables - 1;
    for block in problem.quantifier_blocks.iter().rev() {
        for _ in 0..block.clone() {
            expr = expand(current_quantifier, var, expr);
            let sz = expr.size();
            println!("expanded {} {}", var, sz);

            if sz > 1000000 {
                panic!("expansion failed");
            }

            var -= 1;
        }
        current_quantifier = opposite_quantifier(current_quantifier);
    }

    match *expr {
        RExp::True => Solution::Sat,
        RExp::False => Solution::Unsat,
        _ => panic!("free variable")
    }
}
