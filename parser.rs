use nom::eof;
use nom::alpha;
use nom::space;
use nom::multispace;
use nom::IResult;

use problem::Quantifier;

#[derive(Debug, Clone)]
pub struct Literal {
    pub polarity: bool,
    pub var: String
}

#[derive(Debug, Clone)]
pub enum Expression {
    And(Literal, Literal),
    Or(Literal, Literal),
    Not(Literal),
    Lit(Literal),
    True,
    False
}

#[derive(Debug)]
pub struct Statement {
    pub name: String,
    pub exp: Expression
}

#[derive(Debug)]
pub struct Problem {
    pub quantifiers: Vec<(Quantifier, String)>,
    pub statements: Vec<Statement>,
    pub output: Literal
}

fn string_from_slice(slice: &[u8]) -> String {
    let mut v = Vec::new();
    v.extend_from_slice(slice);
    String::from_utf8(v).unwrap()
}

named!(positive_literal<&[u8], Literal >,
    chain!(
        name: alpha,

        ||{Literal {var: string_from_slice(name), polarity: true}}
    )
);

named!(negative_literal<&[u8], Literal >,
    chain!(
        tag!("~") ~
        name: alpha,

        ||{Literal {var: string_from_slice(name), polarity: false}}
    )
);

named!(literal<&[u8], Literal >,
    alt!(
        positive_literal |
        negative_literal
    )
);

named!(and<&[u8], Expression >,
    chain!(
        tag!("and") ~
        tag!("(") ~
        lhs: literal ~
        tag!(",") ~
        opt!(space) ~
        rhs: literal ~
        tag!(")"),

        ||{Expression::And(lhs, rhs)}
    )
);

named!(or<&[u8], Expression >,
    chain!(
        tag!("or") ~
        tag!("(") ~
        lhs: literal ~
        tag!(",") ~
        opt!(space) ~
        rhs: literal ~
        tag!(")"),

        ||{Expression::Or(lhs, rhs)}
    )
);

named!(not<&[u8], Expression >,
    chain!(
        tag!("not") ~
        tag!("(") ~
        exp: literal ~
        tag!(")"),

        ||{Expression::Not(exp)}
    )
);

named!(true_<&[u8], Expression >,
    chain!(
        tag!("true"),

        ||{Expression::True}
    )
);

named!(false_<&[u8], Expression >,
    chain!(
        tag!("true"),

        ||{Expression::False}
    )
);

named!(literal_expr<&[u8], Expression >,
    chain!(
        lit: literal,
        ||{Expression::Lit(lit)}
    )
);

named!(expression<&[u8], Expression >,
    alt!(and | or | not | true_ | false_ | literal_expr)
);

named!(statement<&[u8], Statement >,
    chain!(
        name: alpha ~
        opt!(space) ~
        tag!("=") ~
        opt!(space) ~
        exp: expression ~
        opt!(multispace),

        ||{Statement { name: string_from_slice(name), exp: exp }}
    )
);

named!(forall_<&[u8], Quantifier >,
    chain!(
        tag!("forall"),

        ||{Quantifier::ForAll}
    )
);

named!(exists_<&[u8], Quantifier >,
    chain!(
        tag!("exists"),

        ||{Quantifier::Exists}
    )
);

named!(quantifier<&[u8], (Quantifier, String) >,
    chain!(
        quantifier:
            alt!(
                forall_ |
                exists_
            ) ~
        multispace ~
        name: alpha ~
        multispace,

        || {(quantifier, string_from_slice(name))}
    )
);

named!(file<&[u8], Problem >,
    chain!(
        quantifiers: many0!(quantifier) ~
        statements: many0!(statement) ~
        output: literal ~
        opt!(multispace) ~
        eof,

        ||{Problem {quantifiers: quantifiers, statements: statements, output: output}}
    )
);

pub fn parse(input: &[u8]) -> Problem {
    match file(input) {
        IResult::Done(_, o) => o,
        o => panic!("failure to parse: {:?}", o)
    }
}
