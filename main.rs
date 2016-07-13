#[macro_use]
extern crate nom;

mod problem;
mod solver;
mod parser;

use std::fs::File;
use std::io::Read;

use problem::Quantifier;
use problem::Expression;
use problem::QBF;

use solver::Solution;
use solver::solve;

fn main() {
    let mut f = File::open("input.qbf").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    println!("{:?}", parser::parse(s.as_ref()));

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
