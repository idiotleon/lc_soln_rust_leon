use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts/
/// Time Complexity:    O(`_len_ns`) ~ O(100) ~ O(1)
/// Space Complexity:   O(`_len_ns`) ~ O(100) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let set: HashSet<i32> = nums.into_iter().collect();
        let len_s: i32 = set.len() as i32;
        if set.contains(&0) {
            len_s - 1
        } else {
            len_s
        }
    }
}
