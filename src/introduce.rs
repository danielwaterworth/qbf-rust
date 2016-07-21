use std::rc::Rc;
use std::collections::HashMap;

use parser;
use problem;
use builder::Builder;

use parser::Statement;
use parser::Expression as PExp;

use problem::Quantifier;

use rc_expression;
use rc_expression::Expression as Exp;

fn lookup_literal(builder: &mut Builder, l: &parser::Literal) -> Rc<Exp> {
    let e = builder.get(&l.var);
    if l.polarity {
        e
    } else {
        builder.not(e)
    }
}

fn build_statements(mut builder: &mut Builder, statements: &[Statement]) {
    for statement in statements {
        let name = &statement.name;
        match &statement.exp {
            &PExp::True => {
                let e = builder.true_();
                builder.set(name.clone(), e);
            },
            &PExp::False => {
                let e = builder.false_();
                builder.set(name.clone(), e);
            },
            &PExp::Not(ref a) => {
                let e = lookup_literal(builder, a);
                let e_ = builder.not(e);
                builder.set(name.clone(), e_);
            },
            &PExp::And(ref a, ref b) => {
                let a1 = lookup_literal(builder, a);
                let b1 = lookup_literal(builder, b);
                let e = builder.and(a1, b1);
                builder.set(name.clone(), e);
            },
            &PExp::Or(ref a, ref b) => {
                let a1 = lookup_literal(builder, a);
                let b1 = lookup_literal(builder, b);
                let e = builder.or(a1, b1);
                builder.set(name.clone(), e);
            },
            &PExp::Lit(ref l) => {
                let e = lookup_literal(builder, l);
                builder.set(name.clone(), e);
            }
        }
    }
}

fn quantifier_blocks(quantifiers: &[Quantifier]) -> (Quantifier, Quantifier, Vec<u32>) {
    if quantifiers.len() == 0 {
        (Quantifier::Exists, Quantifier::Exists, vec![])
    } else {
        let first_quantifier = quantifiers[0].clone();
        let mut output = vec![];

        let mut current_quantifier = first_quantifier.clone();
        let mut n = 1;

        for quantifier in &quantifiers[1..] {
            if quantifier.clone() == current_quantifier {
                n += 1;
            } else {
                current_quantifier = quantifier.clone();
                output.push(n);
                n = 1;
            }
        }
        output.push(n);

        (first_quantifier, current_quantifier, output)
    }
}

pub fn with_parsed_problem<F, X>(parsed: parser::Problem, mut f: F) -> X
    where F : for<'r> FnMut(problem::QBF<'r>) -> X
{
    let quantifiers = parsed.quantifiers;
    let statements = parsed.statements;
    let output = parsed.output;

    let (quantifiers1, names) : (Vec<_>, Vec<_>) = quantifiers.into_iter().unzip();
    let variable_expressions = (0..(quantifiers1.len() as u32)).map(|v| Rc::new(Exp::Var(v)));
    let variables: HashMap<_, _> = names.into_iter().zip(variable_expressions).collect();

    let mut builder = Builder::new(variables);
    build_statements(&mut builder, statements.as_slice());
    let e = lookup_literal(&mut builder, &output);
    let (first_quantifier, last_quantifier, blocks) = quantifier_blocks(&quantifiers1.as_slice());

    rc_expression::with(e, &mut |e1| {
        f(problem::QBF {
            first_quantifier: first_quantifier,
            last_quantifier: last_quantifier,
            quantifier_blocks: blocks.as_slice(),
            expr: e1
        })
    })
}
