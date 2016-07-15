use super::super::introduce::with_parsed_problem;
use super::super::parser;
use super::super::problem;
use problem::Quantifier;

use parser::Expression as PExp;
use problem::Expression as QExp;

fn pos(s: &str) -> parser::Literal {
    parser::Literal { polarity: true, var: s.to_string() }
}

fn st(s: &str, exp: PExp) -> parser::Statement {
    parser::Statement { name: s.to_string(), exp: exp }
}

#[test]
fn false_becomes_false() {
    let p = parser::Problem {
        quantifiers: vec![],
        statements: vec![st("x", PExp::False)],
        output: pos("x")
    };

    let f: &for<'r> Fn(problem::QBF<'r>) -> () = &|qbf| {
        match *qbf.expr {
            QExp::False => (),
            _ => panic!("bad")
        }
    };

    with_parsed_problem(p, f);
}

#[test]
fn true_becomes_true() {
    let p = parser::Problem {
        quantifiers: vec![],
        statements: vec![st("x", PExp::True)],
        output: pos("x")
    };

    let f: &for<'r> Fn(problem::QBF<'r>) -> () = &|qbf| {
        match *qbf.expr {
            QExp::True => (),
            _ => panic!("bad")
        }
    };

    with_parsed_problem(p, f);
}

#[test]
fn or_becomes_or() {
    let p = parser::Problem {
        quantifiers: vec![(Quantifier::ForAll, "x".to_string())],
        statements: vec![st("y", PExp::Or(pos("x"), pos("x")))],
        output: pos("y")
    };

    let f: &for<'r> Fn(problem::QBF<'r>) -> () = &|qbf| {
        match *qbf.expr {
            QExp::Or(&QExp::Var(0), &QExp::Var(0)) => (),
            _ => panic!("bad")
        }
    };

    with_parsed_problem(p, f);
}

#[test]
fn and_becomes_and() {
    let p = parser::Problem {
        quantifiers: vec![(Quantifier::ForAll, "x".to_string())],
        statements: vec![st("y", PExp::And(pos("x"), pos("x")))],
        output: pos("y")
    };

    let f: &for<'r> Fn(problem::QBF<'r>) -> () = &|qbf| {
        match *qbf.expr {
            QExp::And(&QExp::Var(0), &QExp::Var(0)) => (),
            _ => panic!("bad")
        }
    };

    with_parsed_problem(p, f);
}

#[test]
fn not_becomes_not() {
    let p = parser::Problem {
        quantifiers: vec![(Quantifier::ForAll, "x".to_string())],
        statements: vec![st("y", PExp::Not(pos("x")))],
        output: pos("y")
    };

    let f: &for<'r> Fn(problem::QBF<'r>) -> () = &|qbf| {
        match *qbf.expr {
            QExp::Not(&QExp::Var(0)) => (),
            _ => panic!("bad")
        }
    };

    with_parsed_problem(p, f);
}