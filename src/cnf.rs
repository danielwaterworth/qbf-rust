use problem::Expression;

fn tseytin<'r>(expr: &'r Expression<'r>, start_at: i64) -> (Vec<Vec<i64>>, i64) {
    match expr {
        &Expression::And(a, b) => {
            let (mut s_a, a_v) = tseytin(a, start_at);
            let (s_b, b_v) = tseytin(b, a_v.abs() + 1);
            let c_v = b_v.abs() + 1;
            let s_c = vec![vec![c_v, -a_v, -b_v], vec![-c_v, a_v], vec![-c_v, b_v]];
            s_a.extend(s_b);
            s_a.extend(s_c);
            (s_a, c_v)
        },
        &Expression::Or(a, b) => {
            let (mut s_a, a_v) = tseytin(a, start_at);
            let (s_b, b_v) = tseytin(b, a_v.abs() + 1);
            let c_v = b_v.abs() + 1;
            let s_c = vec![vec![-c_v, a_v, b_v], vec![c_v, -a_v], vec![c_v, -b_v]];
            s_a.extend(s_b);
            s_a.extend(s_c);
            (s_a, c_v)
        },
        &Expression::Not(a) => {
            let (s_a, a_v) = tseytin(a, start_at);
            (s_a, -a_v)
        },
        &Expression::True => {
            (vec![], 1)
        },
        &Expression::False => {
            (vec![], -1)
        },
        &Expression::Var(n) => {
            (vec![], (n as i64) + 1)
        }
    }
}
