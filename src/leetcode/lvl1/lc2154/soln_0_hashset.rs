use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/keep-multiplying-found-values-by-two/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let _len_n: usize = nums.len();
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut num = original;
        while num_set.contains(&num) {
            num *= 2;
        }
        num
    }
}
