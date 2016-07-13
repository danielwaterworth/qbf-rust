mod problem;
mod solver;

use problem::Quantifier;
use problem::Expression;
use problem::QBF;

use solver::Solution;
use solver::solve;

fn main() {
    let n = Expression::Var(0);
    let m = Expression::Var(1);
    let e = Expression::And(&n, &m);
    let quantifiers = [Quantifier::Exists, Quantifier::ForAll];
    let q = QBF {
        start_at: 0,
        quantifiers: &quantifiers,
        expr: &e
    };
    match solve(&q) {
        Solution::Sat => println!("sat"),
        Solution::Unsat => println!("unsat")
    }
}
