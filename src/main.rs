#[macro_use]
extern crate nom;

mod solve;
mod problem;
mod substitute;
mod parser;
mod introduce;

use std::fs::File;
use std::io::Read;

use solve::Solution;
use solve::solve;

fn main() {
    std::thread::Builder::new().stack_size(8*1024*1024).spawn(|| {
        let args: Vec<_> = std::env::args().collect();
        if !args.len() < 2 {
            panic!("Expected filename");
        }

        let mut f = File::open(&args[1]).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let parsed = parser::parse(s.as_ref());

        let f: &for<'r> Fn(problem::QBF<'r>) -> () = &|qbf| {
            match solve(&qbf) {
                Solution::Sat => println!("sat"),
                Solution::Unsat => println!("unsat")
            }
        };

        introduce::with_parsed_problem(parsed, f)
    }).unwrap().join();
}
