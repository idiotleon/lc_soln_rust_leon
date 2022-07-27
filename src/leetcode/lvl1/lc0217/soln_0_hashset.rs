use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/contains-duplicate/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(`_len_ns)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let _len_ns: usize = nums.len();
        let mut seen: HashSet<i32> = HashSet::new();
        for num in nums {
            if !seen.insert(num) {
                return true;
            }
        }
        false
    }
}
