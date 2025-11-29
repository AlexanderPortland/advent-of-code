use crate::{pt1, pt2};

#[test]
fn day2_pt1() {
    // Check given example.
    assert_eq!(pt1("given"), 2);

    // Check input still gives what was the correct answer.
    assert_eq!(pt1("input"), 490);
}

#[test]
fn day2_pt2() {
    // Check given example.
    assert_eq!(pt2("given"), 4);

    // Check input still gives what was the correct answer.
    assert_eq!(pt2("input"), 536);
}
