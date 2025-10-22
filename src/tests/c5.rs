use crate::tasks::c5_collections::{
    longest_increasing_subsequence,
    normalize_and_capitalize,
    reverse_words,
    second_largest,
    top_k_frequent,
    unique_chars,
};

#[test]
fn test_second_largest() {
    assert_eq!(second_largest(&[4, -5, 1, -3, 2]), Some(2));
    assert_eq!(second_largest(&[5, 5, 5]), None);
    assert_eq!(second_largest(&[10]), None);
}

#[test]
fn test_longest_increasing_subsequence() {
    assert_eq!(
        longest_increasing_subsequence(&[10, 9, 5, 2, 8, 16, 4, 101]),
        vec![5, 8, 16, 101]
    );
    assert_eq!(longest_increasing_subsequence(&[0, 1, 0, 3, 2, 3]), vec![0, 1, 2, 3]);
}

#[test]
fn test_reverse_words() {
    assert_eq!(reverse_words("Rust is awesome"), "awesome is Rust");
    assert_eq!(reverse_words("Hello world"), "world Hello");
}

#[test]
fn test_normalize_and_capitalize() {
    assert_eq!(normalize_and_capitalize("   rust   is   GREAT   "), "Rust Is Great");
    assert_eq!(normalize_and_capitalize("HELLO   woRLd"), "Hello World");
}

#[test]
fn test_unique_chars() {
    assert!(unique_chars("Rust"));
    assert!(!unique_chars("Programming"));
    assert!(!unique_chars("ТЕТРИДАНОХ"));
    assert!(!unique_chars("ТЕРИАНТРОХ"));
    assert!(!unique_chars("ТЕТРИАНДОХ"));
}

#[test]
fn test_top_k_frequent() {
    assert_eq!(vec![3, 2], top_k_frequent(vec![3, 3, 3, 2, 2, 1], 2));
    assert_eq!(vec![4, 2, 1], top_k_frequent(vec![4, 4, 4, 4, 2, 2, 1], 9));
}
