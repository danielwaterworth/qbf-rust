use std::collections::HashMap;
use std::rc::Rc;

use rc_expression::Expression as RExp;

struct ExpPrinter {
    output: String,
    next_var: u32,
    serialized: HashMap<*const (), String>
}

impl ExpPrinter {
    fn new() -> ExpPrinter {
        ExpPrinter {
            output: "digraph g {\n".to_string(),
            serialized: HashMap::new(),
            next_var: 1
        }
    }

    fn new_var(&mut self) -> String {
        let n = self.next_var;
        self.next_var += 1;
        format!("v{}", n)
    }

    fn build(&mut self, exp: Rc<RExp>) -> String {
        let expr_ptr = &*exp as *const _ as *const ();
        match self.serialized.get(&expr_ptr).map(|v| v.clone()) {
            Some(v) => v.clone(),
            None => {
                let outcome =
                    match *exp {
                        RExp::And(ref a, ref b) => {
                            let a1 = self.build(a.clone());
                            let b1 = self.build(b.clone());
                            let v = self.new_var();
                            self.output.push_str(&format!("  {} [label=\"and\"];\n", &v));
                            self.output.push_str(&format!("  {} -> {};\n", &v, a1));
                            self.output.push_str(&format!("  {} -> {};\n", &v, b1));
                            v
                        },
                        RExp::Not(ref a) => {
                            let a1 = self.build(a.clone());
                            let v = self.new_var();
                            
                            self.output.push_str(&format!("  {} [label=\"not\"];\n", &v));
                            self.output.push_str(&format!("  {} -> {};\n", &v, a1));
                            v
                        },
                        RExp::Var(n) =>
                            format!("arg_{}", n),
                        RExp::True => {
                            let v = self.new_var();
                            self.output.push_str(&format!("  {} [label=\"true\"];\n", &v));
                            v
                        },
                        RExp::False => {
                            let v = self.new_var();
                            self.output.push_str(&format!("  {} [label=\"false\"];\n", &v));
                            v
                        },
                    };
                self.serialized.insert(expr_ptr, outcome.clone());
                outcome
            }
        }
    }
}

pub fn printout(exp: Rc<RExp>) -> String {
    let mut printer = ExpPrinter::new();
    let out = printer.build(exp);
    printer.output.push_str("  out [label=\"output\"];\n");
    printer.output.push_str(&format!("  out -> {}\n", &out));
    printer.output.push_str("}");
    printer.output
}
