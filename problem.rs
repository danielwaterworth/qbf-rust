#[derive(Debug, Copy, Clone)]
pub enum Quantifier {
    Exists,
    ForAll
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

pub static TRUE: Expression<'static> = Expression::True;
pub static FALSE: Expression<'static> = Expression::False;

#[derive(Debug)]
pub struct QBF<'r> {
    pub start_at: u64,
    pub quantifiers: &'r [Quantifier],
    pub expr: &'r Expression<'r>
}
