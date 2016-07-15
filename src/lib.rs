#[macro_use]
extern crate nom;
extern crate picorust;

pub mod solve;
pub mod problem;
pub mod substitute;
pub mod parser;
pub mod introduce;
pub mod cnf;

#[cfg(test)]
mod tests {
    mod introduce;
}
