use crate::{do_calc, pt1, pt2};

#[test]
fn day1_pt1() {
    // Check given example.
    assert_eq!(do_calc("given", pt1), 11);

    // Check input still gives what was the correct answer.
    assert_eq!(do_calc("input", pt1), 1_590_491);
}

#[test]
fn day1_pt2() {
    // Check given example.
    assert_eq!(do_calc("given", pt2), 31);

    // Check input still gives what was the correct answer.
    assert_eq!(do_calc("input", pt2), 22_588_371);
}
