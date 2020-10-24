/*
4. Median of Two Sorted Arrays

Given two sorted arrays nums1 and nums2 of size m and n respectively.

Return the median of the two sorted arrays.

Follow up: The overall run time complexity should be O(log (m+n)). 

Example 1:

Input: nums1 = [1,3], nums2 = [2]
Output: 2.00000
Explanation: merged array = [1,2,3] and median is 2.

Example 2:

Input: nums1 = [1,2], nums2 = [3,4]
Output: 2.50000
Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.

Example 3:

Input: nums1 = [0,0], nums2 = [0,0]
Output: 0.00000

Example 4:

Input: nums1 = [], nums2 = [1]
Output: 1.00000

Example 5:

Input: nums1 = [2], nums2 = []
Output: 2.00000

 

Constraints:

    nums1,length == m
    nums2,length == n
    0 <= m <= 1000
    0 <= n <= 1000
    1 <= m + n <= 2000
*/


fn find_median_sorted_arrays_not_logarithmic(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    fn merge(n1: Vec<i32>, n2: Vec<i32>) -> Vec<i32> {
        let l1 = n1.len();
        let l2 = n2.len();

        let mut i1: usize = 0;
        let mut i2: usize = 0;

        let mut merged: Vec<i32> = Vec::new();

        while i1 < l1 || i2 < l2 {
            match (n1.get(i1), n2.get(i2)) {
                (None, None) => { break; },
                (Some(v1), Some(v2)) if v1 < v2 => {
                    merged.push(*v1);
                    i1 = i1 + 1;
                },
                (Some(v1), None) => {
                    merged.push(*v1);
                    i1 = i1 + 1;
                }
                (Some(_), Some(v2)) | (None, Some(v2)) => {
                    merged.push(*v2);
                    i2 = i2 + 1;
                }
            }
        }

        merged
    }

    let merged = merge(nums1, nums2);
    match merged.len() {
        v if v % 2 == 0 => {
            let i = v/2;
            let l = merged[i-1] as f64;
            let r = merged[i] as f64;

            (l + r) / 2.0
        },
        v => { merged[v/2] as f64 }
    }
}

#[test]
pub fn median_two_sorted_arrays_test() {
    assert_eq!(find_median_sorted_arrays_not_logarithmic(vec![1,3], vec![2]), 2.0);
    assert_eq!(find_median_sorted_arrays_not_logarithmic(vec![1,2], vec![3,4]), 2.5);
    assert_eq!(find_median_sorted_arrays_not_logarithmic(vec![0,0], vec![0,0]), 0.0);
    assert_eq!(find_median_sorted_arrays_not_logarithmic(vec![], vec![1]), 1.0);
    assert_eq!(find_median_sorted_arrays_not_logarithmic(vec![2], vec![]), 2.0);
}