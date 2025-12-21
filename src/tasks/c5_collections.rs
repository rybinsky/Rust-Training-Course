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
}// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    if vec.len() < 2 {
        return None;
    }

    let mut first = i32::MIN;
    let mut second = i32::MIN;

    for &num in vec {
        if num > first {
            second = first;
            first = num;
        } else if num > second && num < first {
            second = num;
        }
    }

    if second == i32::MIN { None } else { Some(second) }
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    if init_sequence.is_empty() {
        return Vec::new();
    }

    let n = init_sequence.len();
    let mut dp = vec![1; n];
    let mut prev = vec![None; n];

    for i in 0..n {
        for j in 0..i {
            if init_sequence[j] < init_sequence[i] && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                prev[i] = Some(j);
            }
        }
    }

    let mut max_len = dp[0];
    let mut max_idx = 0;
    for i in 1..n {
        if dp[i] > max_len {
            max_len = dp[i];
            max_idx = i;
        }
    }

    let mut result = Vec::new();
    let mut current = Some(max_idx);
    while let Some(idx) = current {
        result.push(init_sequence[idx]);
        current = prev[idx];
    }

    result.reverse();
    result
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    words.iter().rev().map(|&s| s).collect::<Vec<&str>>().join(" ")
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    words
        .iter()
        .map(|&word| {
            let mut chars: Vec<char> = word.chars().collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().next().unwrap();
                for i in 1..chars.len() {
                    chars[i] = chars[i].to_lowercase().next().unwrap();
                }
            }
            chars.into_iter().collect()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    let mut seen = HashSet::new();
    for c in s.chars() {
        let lower = c.to_lowercase().collect::<String>();
        for lc in lower.chars() {
            if !seen.insert(lc) {
                return false;
            }
        }
    }
    true
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut freq_map = HashMap::new();
    for &num in &nums {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (num, count) in freq_map {
        heap.push(Reverse((count, num)));
        if heap.len() > k {
            heap.pop();
        }
    }

    heap.into_sorted_vec().into_iter().map(|Reverse((_, num))| num).collect()
}

