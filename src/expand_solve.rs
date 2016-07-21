use std::rc::Rc;

use problem::Quantifier;
use problem::Solution;
use problem::opposite_quantifier;

use rc_expression::Exp as RExp;
use rc_expression::QBF;

use rc_substitute::substitute;

fn expand(quantifier: Quantifier, var: u32, exp: Rc<RExp>) -> Rc<RExp> {
    let false_expr = substitute(exp.clone(), var, false);
    let true_expr = substitute(exp, var, true);
    match quantifier {
        Quantifier::ForAll => RExp::and(false_expr, true_expr),
        Quantifier::Exists => RExp::or(false_expr, true_expr)
    }
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
