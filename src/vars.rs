use std::ops::Range;

use bit_vec::BitVec;

#[derive(Debug, Clone)]
pub struct Vars {
    vars: BitVec
}

impl Vars {
    pub fn new() -> Vars {
        Vars { vars: BitVec::new() }
    }

    fn grow_to(&mut self, n: usize) {
        let m = self.vars.len();
        if m < n {
            self.vars.grow(n - m, false);
        }
    }

    pub fn union(&mut self, b: &mut Vars) {
        self.grow_to(b.vars.len());
        b.grow_to(self.vars.len());
        self.vars.union(&b.vars);
    }

    pub fn add(&mut self, i: u32) {
        self.grow_to((i + 1) as usize);
        self.vars.set(i as usize, true);
    }

    pub fn get(&self, i: u32) -> bool {
        self.vars.get(i as usize).unwrap_or(false)
    }

    pub fn len(&self) -> u32 {
        self.vars.len() as u32
    }
}
