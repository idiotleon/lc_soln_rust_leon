use std::cmp::max;

/// @author: Leon
/// https://leetcode.com/problems/partition-array-for-maximum-sum/
/// Time Complexity:    O(`len_n` * `k`)
/// Space Complexity:   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/partition-array-for-maximum-sum/discuss/290863/JavaC%2B%2BPython-DP-O(K)-Space
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sum_after_partitioning(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut dp: Vec<i32> = vec![0; len_n + 1];
        for idx in 1..=len_n {
            let mut cur_max: i32 = 0;
            let mut max_sum: i32 = 0;
            for par in 1..=k {
                if idx as i32 - par < 0 {
                    break;
                }
                cur_max = max(cur_max, nums[idx - par as usize]);
                max_sum = max(max_sum, dp[idx - par as usize] + cur_max * par);
            }
            dp[idx] = max_sum;
        }
        dp[len_n]
    }
}
