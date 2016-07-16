use std::collections::HashMap;

use parser;
use problem;

use parser::Expression as PExp;
use problem::Expression as QExp;

#[derive(Debug, Copy, Clone)]
struct Exp<'r> {
    e: &'r QExp<'r>,
    e_: &'r QExp<'r>
}

fn not<'r>(exp: Exp<'r>) -> Exp<'r> {
    Exp {e: exp.e_, e_: exp.e}
}

static TRUE: Exp<'static> = Exp{e: &problem::TRUE, e_: &problem::FALSE};
static FALSE: Exp<'static> = Exp{e: &problem::FALSE, e_: &problem::TRUE};

fn with_statement<'r, S, F, X>(
                mut expressions: HashMap<String, Exp<'r>>,
                statements: S,
                name: String,
                exp: Exp<'r>,
                f: F) -> X
    where F : for<'r1> Fn(HashMap<String, Exp<'r1>>, S) -> X,
          S : Iterator<Item=parser::Statement> {
    expressions.insert(name, exp);
    with_statements(expressions, statements, f)
}

fn lookup_literal<'r>(expressions: &HashMap<String, Exp<'r>>, l: parser::Literal) -> Exp<'r> {
    let mut a = expressions.get(&l.var).expect(&l.var).clone();
    if !l.polarity {
        a = not(a);
    }
    return a;
}

fn with_statements<'r, S, F, X>(expressions: HashMap<String, Exp<'r>>, mut statements: S, f: F) -> X
    where F : for<'r1> Fn(HashMap<String, Exp<'r1>>, S) -> X,
          S : Iterator<Item=parser::Statement> {
    match statements.next() {
        Some(statement) => {
            match statement.exp {
                PExp::True =>
                    with_statement(expressions, statements, statement.name, TRUE, f),
                PExp::False =>
                    with_statement(expressions, statements, statement.name, FALSE, f),
                PExp::Not(a) => {
                    let a1 = lookup_literal(&expressions, a);
                    with_statement(expressions, statements, statement.name, not(a1), f)
                },
                PExp::And(a, b) => {
                    let a1 = lookup_literal(&expressions, a);
                    let b1 = lookup_literal(&expressions, b);
                    match (a1.e, b1.e) {
                        (&QExp::False, _) =>
                            with_statement(expressions, statements, statement.name, FALSE, f),
                        (_, &QExp::False) =>
                            with_statement(expressions, statements, statement.name, FALSE, f),
                        (&QExp::True, _) =>
                            with_statement(expressions, statements, statement.name, b1, f),
                        (_, &QExp::True) =>
                            with_statement(expressions, statements, statement.name, a1, f),
                        _ => {
                            let e = problem::and(a1.e, b1.e);
                            let e_ = problem::not(&e);
                            let e1 = Exp { e: &e, e_: &e_ };
                            with_statement(expressions, statements, statement.name, e1, f)
                        }
                    }
                },
                PExp::Or(a, b) => {
                    let a1 = lookup_literal(&expressions, a);
                    let b1 = lookup_literal(&expressions, b);
                    match (a1.e, b1.e) {
                        (&QExp::False, _) =>
                            with_statement(expressions, statements, statement.name, b1, f),
                        (_, &QExp::False) =>
                            with_statement(expressions, statements, statement.name, a1, f),
                        (&QExp::True, _) =>
                            with_statement(expressions, statements, statement.name, TRUE, f),
                        (_, &QExp::True) =>
                            with_statement(expressions, statements, statement.name, TRUE, f),
                        _ => {
                            let e = problem::or(a1.e, b1.e);
                            let e_ = problem::not(&e);
                            let e1 = Exp { e: &e, e_: &e_ };
                            with_statement(expressions, statements, statement.name, e1, f)
                        }
                    }
                },
                PExp::Lit(l) => {
                    let a = lookup_literal(&expressions, l);
                    with_statement(expressions, statements, statement.name, a, f)
                }
            }
        },
        None => {
            f(expressions, statements)
        }
    }
}

pub fn with_parsed_problem<F, X>(mut parsed: parser::Problem, f: F) -> X
    where F : for<'r> Fn(problem::QBF<'r>) -> X {
    let mut quantifiers = parsed.quantifiers;
    let statements = parsed.statements.drain(..);
    let output = parsed.output;

    let (quantifiers1, mut names) : (Vec<_>, Vec<_>) = quantifiers.drain(..).unzip();
    let variable_expressions: Vec<_> = (0..(quantifiers1.len() as u64)).map(QExp::Var).collect();
    let complements: Vec<_> = variable_expressions.iter().map(problem::not).collect();
    let merged = variable_expressions.iter().zip(complements.iter()).map(|(e, e_)| Exp {e: e, e_: e_});
    let expressions: HashMap<_, _> = names.drain(..).zip(merged).collect();

    let g: &for<'r1> Fn(HashMap<String, Exp<'r1>>, _) -> X = &|expressions, _| {
        f(problem::QBF {
            quantifiers: quantifiers1.as_slice(),
            expr: lookup_literal(&expressions, output.clone()).e
        })
    };

    with_statements(expressions, statements, g)
}
