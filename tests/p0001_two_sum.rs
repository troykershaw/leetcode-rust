/*
1. Two Sum

Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

Example:

Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
*/


use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 { panic!("Leetcode, don't give me vectors that are too small...") };

    let mut diffs: HashMap<i32, usize> = HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        match diffs.get(&n) {
            Some(&i0) => return vec![i0 as i32, i as i32],
            None => { diffs.insert(&target - n, i); }
        }
    }
    
    panic!("Leetcode said this could never happen")
}

#[test]
fn two_sum_test() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
