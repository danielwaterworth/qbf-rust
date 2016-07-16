#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Quantifier {
    Exists,
    ForAll
}

pub fn opposite_quantifier(q: Quantifier) -> Quantifier {
    match q {
        Quantifier::Exists => Quantifier::ForAll,
        Quantifier::ForAll => Quantifier::Exists
    }
}

#[derive(Debug)]
pub enum Expression<'r> {
    And(&'r Expression<'r>, &'r Expression<'r>),
    Or(&'r Expression<'r>, &'r Expression<'r>),
    Not(&'r Expression<'r>),
    Var(u64),
    True,
    False
}

pub fn and<'r>(a: &'r Expression<'r>, b: &'r Expression<'r>) -> Expression<'r> {
    Expression::And(a, b)
}

pub fn or<'r>(a: &'r Expression<'r>, b: &'r Expression<'r>) -> Expression<'r> {
    Expression::Or(a, b)
}

pub fn not<'r>(a: &'r Expression<'r>) -> Expression<'r> {
    Expression::Not(a)
}

pub static TRUE: Expression<'static> = Expression::True;
pub static FALSE: Expression<'static> = Expression::False;

#[derive(Debug)]
pub struct QBF<'r> {
    pub quantifiers: &'r [Quantifier],
    pub expr: &'r Expression<'r>
}
