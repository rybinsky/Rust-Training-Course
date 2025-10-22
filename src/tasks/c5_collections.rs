// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::collections::{HashMap, HashSet};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    !unimplemented!()
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    !unimplemented!()
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    !unimplemented!()
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    !unimplemented!()
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    !unimplemented!()
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    !unimplemented!()
}
