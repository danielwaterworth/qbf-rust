use problem::Quantifier;
use problem::Expression;
use problem::QBF;
use problem::Solution;
use problem::opposite_quantifier;

use substitute::substitute;

fn solve_inner<'r>(
            mut current_quantifier: Quantifier,
            mut current_block: u64,
            mut blocks: &[u64],
            start_at: u64,
            expr: &'r Expression<'r>
        ) -> Solution {
    match expr {
        &Expression::True => return Solution::Sat,
        &Expression::False => return Solution::Unsat,
        _ => {}
    };

    if current_block == 0 {
        if blocks.len() == 0 {
            panic!("free variable")
        } else {
            current_quantifier = opposite_quantifier(current_quantifier);
            current_block = blocks[0];
            blocks = &blocks[1..];
        }
    };

    if !expr.has_var(start_at) {
        return
            solve_inner(
                current_quantifier,
                current_block - 1,
                blocks,
                start_at + 1,
                expr
            );
    }

    let solve_with = |b| {
        substitute(expr, start_at, b, |expr1| {
            solve_inner(
                current_quantifier,
                current_block - 1,
                blocks,
                start_at + 1,
                expr1
            )
        })
    };

    match current_quantifier {
        Quantifier::ForAll => {
            match solve_with(false) {
                Solution::Sat => {
                    solve_with(true)
                },
                Solution::Unsat => Solution::Unsat
            }
        },
        Quantifier::Exists => {
            match solve_with(false) {
                Solution::Sat => Solution::Sat,
                Solution::Unsat => {
                    solve_with(true)
                }
            }
        }
    }
}

pub fn solve<'r>(problem: &'r QBF<'r>) -> Solution {
    solve_inner(
        problem.first_quantifier,
        problem.quantifier_blocks[0],
        &problem.quantifier_blocks[1..],
        0,
        problem.expr
    )
}
