use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/most-frequent-number-following-key-in-an-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut num_to_freq: HashMap<i32, i32> = HashMap::with_capacity(len_ns);
        for idx in 0..len_ns - 1 {
            if nums[idx] == key {
                *num_to_freq.entry(nums[idx + 1]).or_default() += 1;
            }
        }
        num_to_freq
            .into_iter()
            .max_by_key(|entry| entry.1)
            .unwrap()
            .0
    }
}
