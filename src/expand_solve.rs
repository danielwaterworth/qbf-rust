use problem;
use problem::Quantifier;
use problem::Expression;
use problem::QBF;
use problem::Solution;
use problem::opposite_quantifier;

use substitute::substitute;

fn expand_inner<'a, X>(
            expr: &'a Expression<'a>,
            mut current_quantifier: Quantifier,
            mut current_block: u64,
            mut blocks: &[u64],
            start_at: u64,
            mut f: &mut (for<'b> FnMut(&'b Expression<'b>) -> X + 'a)
        ) -> X {
    if current_block == 0 {
        if blocks.len() == 0 {
            return f(expr)
        } else {
            current_quantifier = opposite_quantifier(current_quantifier);
            current_block = blocks[0];
            blocks = &blocks[1..];
        }
    };

    expand_inner(
        expr,
        current_quantifier,
        current_block - 1,
        blocks,
        start_at + 1,
        &mut |expr1| {
            substitute(expr1, start_at, false, |false_expr| {
                substitute(expr1, start_at, true, |true_expr| {
                    match current_quantifier {
                        Quantifier::ForAll => {
                            problem::and(&true_expr, &false_expr, &mut f)
                        },
                        Quantifier::Exists => {
                            problem::or(&true_expr, &false_expr, &mut f)
                        }
                    }
                })
            })
        }
    )
}

pub fn solve<'r>(problem: &'r QBF<'r>) -> Solution {
    expand_inner(
        problem.expr,
        problem.first_quantifier,
        problem.quantifier_blocks[0],
        &problem.quantifier_blocks[1..],
        0,
        &mut |expr| {
            match expr {
                &Expression::True => Solution::Sat,
                &Expression::False => Solution::Unsat,
                _ => panic!("free variable")
            }
        }
    )
}
