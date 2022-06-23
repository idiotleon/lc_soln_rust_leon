use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let len_n: usize = nums.len();
        let target = {
            let mut target = -x;
            let total_sum: i32 = nums.iter().sum();
            target += total_sum;
            target
        };
        if 0 == target {
            return len_n as i32;
        }
        let mut prefix_sum: HashMap<i32, i32> = HashMap::new();
        prefix_sum.insert(0, -1);
        let mut sum: i32 = 0;
        let mut longest: i32 = i32::MIN;
        for (idx, &num) in nums.iter().enumerate() {
            sum += num;
            if let Some(idx_prev) = prefix_sum.get(&(sum - target)) {
                longest = std::cmp::max(longest, (idx as i32) - idx_prev);
            }
            prefix_sum.insert(sum, idx as i32);
        }
        if longest == i32::MIN {
            -1
        } else {
            len_n as i32 - longest
        }
    }
}
