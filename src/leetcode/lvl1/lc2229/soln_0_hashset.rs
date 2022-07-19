use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/check-if-an-array-is-consecutive/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_consecutive(nums: Vec<i32>) -> bool {
        let len_ns: usize = nums.len();
        let max: i32 = *nums.iter().max().unwrap();
        let min: i32 = *nums.iter().min().unwrap();
        if len_ns as i32 != max - min + 1 {
            return false;
        }
        let set: HashSet<i32> = nums.into_iter().collect();
        len_ns == set.len()
    }
}
