use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/maximum-size-subarray-sum-equals-k/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let _len_n = nums.len();
        let mut sum_to_first_idx: HashMap<i32, i32> = HashMap::new();
        sum_to_first_idx.insert(0, -1);
        let mut sum: i32 = 0;
        let mut longest: i32 = 0;
        for (idx, num) in nums.into_iter().enumerate() {
            sum += num;
            if let Some(&prev_idx) = sum_to_first_idx.get(&(sum - k)) {
                let cur_len = idx as i32 - prev_idx;
                longest = std::cmp::max(longest, cur_len);
            }
            sum_to_first_idx.entry(sum).or_insert(idx as i32);
        }
        longest
    }
}
