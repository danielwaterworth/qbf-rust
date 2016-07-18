use problem::Quantifier;
use problem::Expression;
use problem::QBF;
use problem::opposite_quantifier;

use substitute::substitute;

#[derive(Debug)]
pub enum Solution {
    Sat,
    Unsat
}

fn quantifier_blocks(quantifiers: &[Quantifier]) -> (Quantifier, Vec<u64>) {
    let first_quantifier = quantifiers[0].clone();
    let mut output = vec![];

    let mut current_quantifier = first_quantifier.clone();
    let mut n = 1;

    for quantifier in &quantifiers[1..] {
        if quantifier.clone() == current_quantifier {
            n += 1;
        } else {
            output.push(n);
            n = 1;
            current_quantifier = quantifier.clone();
        }
    }
    output.push(n);

    return (first_quantifier, output);
}

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
    let (first_quantifier, blocks) = quantifier_blocks(&problem.quantifiers);
    solve_inner(first_quantifier, blocks[0], &blocks[1..], 0, problem.expr)
}
