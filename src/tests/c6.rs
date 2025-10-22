use crate::tasks::c6_error_handling_tests_docs::{UserProfile, first_char, read_numbers_from_str};

#[test]
fn test_first_char() {
    assert_eq!(first_char("Rust"), Ok('R'));
    assert_eq!(first_char(""), Err("Empty string".to_string()));
}

#[test]
fn test_read_numbers_from_str() {
    assert_eq!(read_numbers_from_str("  2  4 8 20 "), Ok(vec![2, 4, 8, 20]));
    assert_eq!(read_numbers_from_str("  2  4 CHEEKY BREEKY 20 "), Err("Invalid number".into()));
}

#[test]
fn test_get_email_domain() {
    let user_with_email =
        UserProfile::new("alice".to_string(), Some("alice@example.com".to_string()));
    assert_eq!(user_with_email.get_email_domain(), Some("example.com".to_string()));

    let user_without_email = UserProfile::new("bob".to_string(), None);
    assert_eq!(user_without_email.get_email_domain(), None);
}
