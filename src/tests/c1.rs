use crate::tasks::c1_common_concepts::{factorial, find_biggest_number, sign_checker, square};

#[test]
fn test_square() {
    assert_eq!(4, square(2));
    assert_eq!(144, square(12));
}

#[test]
fn test_factorial() {
    assert_eq!(2, factorial(2));
    assert_eq!(120, factorial(5));
    assert_eq!(40320, factorial(8));
}

#[test]
fn test_simple_control_flow() {
    assert_eq!("negative", sign_checker(-5));
    assert_eq!("zero", sign_checker(0));
    assert_eq!("positive", sign_checker(10));
}

#[test]
fn test_find_biggest_number() {
    assert_eq!(99, find_biggest_number([25, 3, 99, 56, 42]));
    assert_eq!(5, find_biggest_number([5, 4, 3, 2, 1]));
    assert_eq!(9, find_biggest_number([5, 6, 7, 8, 9]));
}
