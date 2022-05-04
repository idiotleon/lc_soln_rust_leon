use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/max-number-of-k-sum-pairs/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let _len_n: usize = nums.len();
        let mut num_to_freq: HashMap<i32, u32> = HashMap::new();
        let mut cnt: i32 = 0;
        for num in nums {
            let expected = num_to_freq.entry(k - num).or_default();
            if *expected > 0 {
                *expected -= 1;
                cnt += 1;
            } else {
                *num_to_freq.entry(num).or_default() += 1;
            }
        }
        cnt
    }
}
