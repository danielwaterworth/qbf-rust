#[macro_use]
extern crate nom;
extern crate picorust;
extern crate bit_vec;

mod vars;
pub mod solve;
pub mod problem;
pub mod substitute;
pub mod parser;
pub mod introduce;
pub mod sat;

#[cfg(test)]
mod tests {
    mod introduce;
}
