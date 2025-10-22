use crate::tasks::c3_ownership_and_memory::{last_word, longest_word};

#[test]
fn test_last_word() {
    assert_eq!("kids", last_word("How do you do fellow kids"));
    assert_eq!("end", last_word(" tricky string with a space at the beginning and end "));
    assert_eq!("spaces", last_word("string     with     excess        spaces"));
    assert_eq!("", last_word(""));
}

#[test]
fn test_longest_word() {
    assert_eq!("there", longest_word("I can't feel my Rust! Bubba its aint there"));
    assert_eq!("Somebody", longest_word("Somebody once told me the world is gonna roll me"));
    assert_eq!("gonna", longest_word("Never gonna give you up Never gonna let you down"));
}
