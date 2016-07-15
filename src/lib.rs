#[macro_use]
extern crate nom;

pub mod solve;
pub mod problem;
pub mod substitute;
pub mod parser;
pub mod introduce;

#[cfg(test)]
mod tests {
    mod introduce;
}