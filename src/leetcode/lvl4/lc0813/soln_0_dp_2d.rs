/// @author: Leon
/// https://leetcode.com/problems/largest-sum-of-averages/
/// Time Complexity:    O(`len_n` * `k`)
/// Space Complexity:   O(`len_n` * `k`)
/// Reference:
/// https://leetcode.com/problems/largest-sum-of-averages/discuss/126280/Naive-Detailed-Step-by-Step-Approach-from-Recursive-to-DP-O(N)-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let len_n: usize = nums.len();
        let k: usize = k as usize;
        let prefix_sums: Vec<i32> = {
            let mut prefix_sums: Vec<i32> = vec![0; len_n];
            for idx in 0..len_n {
                prefix_sums[idx] = if idx == 0 { 0 } else { prefix_sums[idx - 1] } + nums[idx];
            }
            prefix_sums
        };
        let mut dp: Vec<Vec<f64>> = vec![vec![0.0; k + 1]; len_n];
        // count of partitions
        for cnt_prts in 1..=k {
            for lo in 0..=len_n - cnt_prts {
                if cnt_prts == 1 {
                    dp[lo][cnt_prts] = (prefix_sums[len_n - 1] - prefix_sums[lo] + nums[lo]) as f64
                        / (len_n - lo) as f64;
                } else {
                    for hi in lo..=len_n - cnt_prts {
                        dp[lo][cnt_prts] = dp[lo][cnt_prts].max(
                            dp[hi + 1][cnt_prts - 1]
                                + (prefix_sums[hi] - prefix_sums[lo] + nums[lo]) as f64
                                    / (hi - lo + 1) as f64,
                        );
                    }
                }
            }
        }
        dp[0][k]
    }
}
