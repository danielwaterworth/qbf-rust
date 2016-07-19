use std::collections::HashSet;

use problem::Expression as QExp;

pub fn expansion_is_cheap<'a>(exp: &'a QExp<'a>, variable: u32, max_cost: u32) -> bool {
    let mut rebuilds = HashSet::new();
    let mut cost = 0;

    let mut to_visit = vec![exp];
    while let Some(node) = to_visit.pop() {
        if cost > max_cost {
            return false;
        }

        let expr_ptr = node as (*const _);
        if !rebuilds.contains(&expr_ptr) && node.has_var(variable) {
            rebuilds.insert(expr_ptr);
            cost += 1;
            match node {
                &QExp::And(_, ref a, ref b) => {
                    to_visit.push(a);
                    to_visit.push(b);
                },
                _ => {}
            }
        }
    }

    true
}
