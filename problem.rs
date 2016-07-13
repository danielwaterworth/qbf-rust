#[derive(Debug)]
pub enum Quantifier {
    Exists,
    ForAll
}

#[derive(Debug)]
pub enum Expression<'r> {
    And(&'r Expression<'r>, &'r Expression<'r>),
    Or(&'r Expression<'r>, &'r Expression<'r>),
    Not(&'r Expression<'r>),
    Var(u32),
    True,
    False
}

#[derive(Debug)]
pub struct QBF<'r> {
    pub start_at: u32,
    pub quantifiers: &'r [Quantifier],
    pub expr: &'r Expression<'r>
}
