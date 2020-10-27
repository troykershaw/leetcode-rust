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

fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 { panic!("'nums' must have at least two items.") };
    for i in 0..nums.len()-1 {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]
            }
        }
    }

    panic!("No items sum to the target")
}

#[test]
fn two_sum_naive_test() {
    assert_eq!(two_sum_naive(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum_naive(vec![2, 5, 3], 8), vec![1, 2]);
    assert_eq!(two_sum_naive(vec![2, 9, 4, 3, -6], -2), vec![2, 4]);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 { panic!("'nums' must have at least two items.") };

    let mut prev: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    for (i, n) in nums.iter().enumerate() {
        if let Some(p) = prev.get(&(target - n)) {
            return vec![*p as i32, i as i32]
        }

        prev.insert(*n, i);
    }
    
    panic!("No items sum to the target")
}

#[test]
fn two_sum_test() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![2, 5, 3], 8), vec![1, 2]);
    assert_eq!(two_sum(vec![2, 9, 4, 3, -6], -2), vec![2, 4]);
}
