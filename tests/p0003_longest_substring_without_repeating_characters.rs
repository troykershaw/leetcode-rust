/*
3. Longest Substring Without Repeating Characters

Given a string, find the length of the longest substring without repeating characters.

Example 1:

Input: "abcabcbb"
Output: 3 
Explanation: The answer is "abc", with the length of 3. 

Example 2:

Input: "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:

Input: "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3. 
             Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
*/

use std::collections::HashSet;
use std::cmp::max;

fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();

    if chars.len() < 2 { return chars.len() as i32}

    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut longest: usize = 0;

    // We're dealing with the lowercase English alphabet so we can allocate
    // our set to the max size we'll ever need.
    let mut seen: HashSet<char> = HashSet::with_capacity(26);

    while r < chars.len() {
        if seen.contains(&chars[r]) {
            seen.remove(&chars[l]);
            l += 1;
        }
        else {
            longest = max(longest, r - l);
            seen.insert(chars[r]);
            r += 1;
        }
    }

    (longest + 1) as i32
}

#[test]
fn length_of_longest_substring_test() {
    assert_eq!(length_of_longest_substring("".to_string()), 0);
    assert_eq!(length_of_longest_substring("abcdabcbd".to_string()), 4);
    assert_eq!(length_of_longest_substring("aaaaaa".to_string()), 1);
    assert_eq!(length_of_longest_substring("tidddosa".to_string()), 4);
    assert_eq!(length_of_longest_substring("au".to_string()), 2);
    assert_eq!(length_of_longest_substring("abcabc".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring("au".to_string()), 2);
}
