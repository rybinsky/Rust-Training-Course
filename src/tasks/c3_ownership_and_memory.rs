// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.
//
// You can implement the function and use it right inside the `string_ownership` function.
#[allow(dead_code)]
pub fn string_ownership() {
    fn longest_owned(s1: String, s2: String) -> String {
        if s1.len() >= s2.len() { s1 } else { s2 }
    }

    let a = String::from("hello");
    let b = String::from("world!!!");
    let result = longest_owned(a, b);
    println!("Longest: {}", result);
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.
#[allow(dead_code)]
pub fn simple_borrowing() {
    fn print_length(s: &String) {
        println!("Length = {}", s.len());
    }

    let text = String::from("borrow me!");
    print_length(&text);
    println!("Still usable: {}", text);
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.
#[allow(dead_code)]
pub fn hard_borrowing() {
    fn append_and_return_length(string: &mut String, suffix: &str) -> usize {
        string.push_str(suffix);
        string.len()
    }

    let mut msg = String::from("Hi");
    let len1 = append_and_return_length(&mut msg, ", there");
    println!("After first append: {}, length = {}", msg, len1);

    let len2 = append_and_return_length(&mut msg, "!");
    println!("After second append: {}, length = {}", msg, len2);
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    slice.split_whitespace().last().unwrap_or("")
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    sentence.split_whitespace().fold(
        "",
        |longest, word| {
            if word.len() >= longest.len() { word } else { longest }
        },
    )
}
