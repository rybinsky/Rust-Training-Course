// This chapter is dedicated to the iterators and closures.

use std::collections::HashMap;

// ITERATORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `word_frequencies(text: &str) -> Vec<(String, usize)>` that:
// - Splits the input text into words.
// - Normalizes them to lowercase.
// - Counts how many times each word appears.
// - Returns the result as a vector of `(word, count)` tuples, sorted by descending frequency.
// If some words have the same frequency, return them in alphabetical order.

pub fn word_frequencies(text: &str) -> Vec<(String, usize)> {
    !unimplemented!()
}

// ----- 2 --------------------------------------
// Write a function `top_k_most_common_letters(text: &str, k: usize) -> Vec<(char, usize)>` that:
// - Counts the frequency of letters only (ignore spaces/punctuation).
// - Case-insensitive.
// - Sorts by descending frequency.
// - Returns the vector with top `k` letters.
// If some letters have the same frequency, return them in alphabetical order.

pub fn top_k_most_common_letters(text: &str, k: usize) -> Vec<(char, usize)> {
    !unimplemented!()
}

// CLOSURES
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function
// `filter_and_sort_names(names: Vec<String>, minimum_length: usize) -> Vec<String>` that:
// - Filters out names shorter than minimum_length.
// - Sorts the remaining names alphabetically (case-insensitive).
// - Returns the result.
// You must use closures in filtering and sorting.

pub fn filter_and_sort_names(names: Vec<String>, minimum_length: usize) -> Vec<String> {
    !unimplemented!()
}

// ----- 4 --------------------------------------
// Create a function `group_students_by_grade` that:
// - Accepts a `Vec<(String, u32)>` where each tuple is `(student_name, grade)`.
// - Groups students into some map where the key is the grade and the value is a vector of names.
// - Uses closures for grouping logic.
// - Returns the grouped map, sorted internally by student names.

pub fn group_students_by_grade(students: Vec<(String, u32)>) -> HashMap<u32, Vec<String>> {
    !unimplemented!()
}
