use std::collections::HashMap;

use parser;
use problem;
use builder::Builder;

use parser::Statement;
use parser::Expression as PExp;

use problem::Expression as QExp;
use problem::Quantifier;

fn lookup_literal<'r, X>(
        builder: Builder<'r>,
        l: &parser::Literal,
        f: &mut (for<'r1> FnMut(Builder<'r1>, &'r1 QExp<'r1>) -> X + 'r)) -> X
{
    let e = builder.get(&l.var);
    if l.polarity {
        f(builder, e)
    } else {
        builder.not(e, f)
    }
}

fn with_statements<'r, X>(
        mut builder: Builder<'r>,
        statements: &[Statement],
        f: &mut (for<'r1> FnMut(Builder<'r1>) -> X + 'r)) -> X
{
    if statements.len() == 0 {
        f(builder)
    } else {
        let next_statements = &statements[1..];
        let statement = &statements[0];
        let name = &statement.name;
        match &statement.exp {
            &PExp::True => {
                let e = &problem::TRUE;
                builder.set(name.clone(), e);
                with_statements(builder, next_statements, f)
            },
            &PExp::False => {
                let e = &problem::FALSE;
                builder.set(name.clone(), e);
                with_statements(builder, next_statements, f)
            },
            &PExp::Not(ref a) => {
                lookup_literal(builder, a, &mut |builder1, e| {
                    builder1.not(e, &mut |mut builder2, e| {
                        builder2.set(name.clone(), e);
                        with_statements(builder2, next_statements, f)
                    })
                })
            },
            &PExp::And(ref a, ref b) => {
                lookup_literal(builder, a, &mut |builder1, a1| {
                    lookup_literal(builder1, b, &mut |builder2, b1| {
                        builder2.and(a1, b1, &mut |mut builder3, e| {
                            builder3.set(name.clone(), e);
                            with_statements(builder3, next_statements, f)
                        })
                    })
                })
            },
            &PExp::Or(ref a, ref b) => {
                lookup_literal(builder, a, &mut |builder1, a1| {
                    lookup_literal(builder1, b, &mut |builder2, b1| {
                        builder2.or(a1, b1, &mut |mut builder3, e| {
                            builder3.set(name.clone(), e);
                            with_statements(builder3, next_statements, f)
                        })
                    })
                })
            },
            &PExp::Lit(ref l) => {
                lookup_literal(builder, l, &mut |mut builder1, e| {
                    builder1.set(name.clone(), e);
                    with_statements(builder1, next_statements, f)
                })
            }
        }
    }
}

fn quantifier_blocks(quantifiers: &[Quantifier]) -> (Quantifier, Vec<u32>) {
    if quantifiers.len() == 0 {
        (Quantifier::Exists, vec![])
    } else {
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

        (first_quantifier, output)
    }
}

pub fn with_parsed_problem<F, X>(parsed: parser::Problem, mut f: F) -> X
    where F : for<'r> FnMut(problem::QBF<'r>) -> X
{
    let mut quantifiers = parsed.quantifiers;
    let statements = parsed.statements;
    let output = parsed.output;

    let (quantifiers1, mut names) : (Vec<_>, Vec<_>) = quantifiers.drain(..).unzip();
    let variable_expressions: Vec<_> = (0..(quantifiers1.len() as u32)).map(QExp::Var).collect();
    let variables: HashMap<_, _> = names.drain(..).zip(variable_expressions).collect();
    let ref_variables = variables.iter().map(|(k, v)| (k.clone(), v)).collect();

    let builder = Builder::new(ref_variables);
    with_statements(builder, statements.as_slice(), &mut |builder1| {
        lookup_literal(builder1, &output, &mut |_, e| {
            let (first_quantifier, blocks) =
                quantifier_blocks(&quantifiers1.as_slice());

            f(problem::QBF {
                first_quantifier: first_quantifier,
                quantifier_blocks: blocks.as_slice(),
                expr: e
            })
        })
    })
}
