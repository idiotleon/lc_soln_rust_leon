/// https://leetcode.com/problems/maximum-size-subarray-sum-equals-k/
///
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
#[allow(dead_code)]
use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        // not used
        // let len_n = nums.len();
        let mut sum_to_first_idx = HashMap::<i32, i32>::new();
        sum_to_first_idx.insert(0, -1);
        let mut sum: i32 = 0;
        let mut longest: i32 = 0;
        for (idx, &num) in nums.iter().enumerate() {
            sum += num;
            if let Some(prev_idx) = sum_to_first_idx.get(&(sum - k)) {
                let cur_len = idx as i32 - prev_idx;
                longest = std::cmp::max(longest, cur_len);
            }
            if !sum_to_first_idx.contains_key(&sum) {
                sum_to_first_idx.insert(sum, idx as i32);
            }
        }
        longest
    }
}
