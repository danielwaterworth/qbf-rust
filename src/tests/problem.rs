use super::super::problem;
use problem::Expression as QExp;

#[test]
fn true_is_not_false() {
    assert!(problem::TRUE != problem::FALSE);
}

#[test]
fn true_is_true() {
    assert_eq!(problem::TRUE, problem::TRUE);
}

#[test]
fn different_trues_are_equal() {
    let t1 = QExp::True;
    let t2 = QExp::True;
    assert_eq!(t1, t2);
    assert_eq!(&t1, &t2);
}

#[test]
fn different_falses_are_equal() {
    let t1 = QExp::False;
    let t2 = QExp::False;
    assert_eq!(t1, t2);
    assert_eq!(&t1, &t2);
}

#[test]
fn false_is_false() {
    assert_eq!(problem::FALSE, problem::FALSE);
}

#[test]
fn not_false_is_true() {
    problem::not(&problem::FALSE, |e| {
        assert_eq!(e, &problem::TRUE)
    });
}

#[test]
fn not_true_is_false() {
    problem::not(&problem::TRUE, |e| {
        assert_eq!(e, &problem::FALSE)
    });
}

#[test]
fn var_is_var() {
    let a = QExp::Var(0);

    assert_eq!(a, a);
}

#[test]
fn var_is_not_var() {
    let a = QExp::Var(0);
    let b = QExp::Var(1);

    assert!(a != b);
}

#[test]
fn different_nots_are_equal() {
    let a = QExp::Var(0);
    let a_1 = QExp::Not(&a);
    let a_2 = QExp::Not(&a);

    assert_eq!(a_1, a_2);
}

#[test]
fn not_not_var_is_var() {
    let a = QExp::Var(0);

    problem::not(&a, |a_| {
        problem::not(a_, |a__| {
            assert_eq!(&a, a__);
        })
    })
}

#[test]
fn or_expr() {
    let a = QExp::Var(0);
    let b = QExp::Var(1);

    let a_ = QExp::Not(&a);

    problem::or(&a_, &b, |e| {
        problem::not(e, |e_1| {
            problem::not(&b, |b_| {
                problem::and(&a, b_, |e_2| {
                    assert_eq!(e_1, e_2);
                })
            })
        })
    })
}
