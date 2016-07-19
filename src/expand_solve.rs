use std::rc::Rc;

use problem;
use problem::QBF;
use problem::Quantifier;
use problem::Solution;
use problem::opposite_quantifier;

use rc_expression::Expression as RExp;
use rc_expression::construct;
use rc_expression::with;

use substitute::substitute;

use solve::solve as enumeration_solve;

fn expand(quantifier: Quantifier, var: u32, exp: Rc<RExp>) -> (Rc<RExp>, usize) {
    with(exp, &mut |exp1| {
        substitute(exp1, var, false, |false_expr| {
            substitute(exp1, var, true, |true_expr| {
                match quantifier {
                    Quantifier::ForAll => {
                        problem::and(false_expr, true_expr, |expr| {
                            (construct(expr), expr.size())
                        })
                    },
                    Quantifier::Exists => {
                        problem::or(false_expr, true_expr, |expr| {
                            (construct(expr), expr.size())
                        })
                    }
                }
            })
        })
    })
}

pub fn solve<'r>(problem: &'r QBF<'r>) -> Solution {
    let n_variables: u32 = problem.quantifier_blocks.iter().sum();
    let mut expr = construct(problem.expr);

    let mut current_quantifier = problem.last_quantifier;
    let mut var = n_variables - 1;
    for block in problem.quantifier_blocks.iter().rev() {
        for _ in 0..block.clone() {
            let (expr1, sz) = expand(current_quantifier, var, expr);
            expr = expr1;

            if sz > 2000 {
                break;
            }

            var -= 1;
        }
        current_quantifier = opposite_quantifier(current_quantifier);
    }

    with(expr, &mut |expr1| {
        enumeration_solve(
            &QBF {
                first_quantifier: problem.first_quantifier.clone(),
                last_quantifier: problem.last_quantifier.clone(),
                quantifier_blocks: problem.quantifier_blocks,
                expr: expr1
            }
        )
    })
}
