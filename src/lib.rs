#[macro_use]
extern crate nom;
extern crate bit_vec;
extern crate picorust;

pub mod builder;
pub mod dot;
pub mod expand_solve;
pub mod expansion;
pub mod introduce;
pub mod n_expression;
pub mod parser;
pub mod printout;
pub mod problem;
pub mod rc_expression;
pub mod sat;
pub mod simplify;
pub mod solve;
pub mod substitute;
pub mod vars;

#[cfg(test)]
mod tests {
    mod introduce;
    mod problem;
}
