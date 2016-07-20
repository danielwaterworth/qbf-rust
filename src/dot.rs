use std::collections::HashMap;
use std::rc::Rc;

use n_expression::Expression as NExp;

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

    fn build_and(&mut self, exps: &[Rc<NExp>]) -> String {
        let v = self.new_var();
        let mut vars = vec![];
        for exp in exps {
            vars.push(self.build(exp.clone()));
        }
        self.output.push_str(&format!("  {} [label=\"and\"];\n", &v));
        for var in vars {
            self.output.push_str(&format!("  {} -> {};\n", &v, &var));
        }
        v
    }

    fn build(&mut self, exp: Rc<NExp>) -> String {
        let expr_ptr = &*exp as *const _ as *const ();
        match self.serialized.get(&expr_ptr).map(|v| v.clone()) {
            Some(v) => v.clone(),
            None => {
                let outcome =
                    match *exp {
                        NExp::And(ref x) => {
                            self.build_and(x)
                        },
                        NExp::Not(ref a) => {
                            let a1 = self.build(a.clone());
                            let v = self.new_var();
                            self.output.push_str(&format!("  {} [label=\"not\"];\n", &v));
                            self.output.push_str(&format!("  {} -> {};\n", &v, a1));
                            v
                        },
                        NExp::Var(n) =>
                            format!("arg_{}", n),
                        NExp::True => {
                            let v = self.new_var();
                            self.output.push_str(&format!("  {} [label=\"true\"];\n", &v));
                            v
                        },
                        NExp::False => {
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

pub fn printout(exp: Rc<NExp>) -> String {
    let mut printer = ExpPrinter::new();
    let out = printer.build(exp);
    printer.output.push_str("  out [label=\"output\"];\n");
    printer.output.push_str(&format!("  out -> {}\n", &out));
    printer.output.push_str("}");
    printer.output
}
