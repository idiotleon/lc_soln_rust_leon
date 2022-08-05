/// @author: Leon
/// https://leetcode.com/problems/combination-sum-iv/
/// Time Complexity:    O(`_len_ns` * `target`)
/// Space Complexity:   O(`target`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let _len_ns: usize = nums.len();
        let mut dp: Vec<i32> = {
            let mut dp: Vec<i32> = vec![0; target as usize + 1];
            dp[0] = 1;
            dp
        };
        for sum in 1..=target {
            for &num in &nums {
                if sum >= num {
                    dp[sum as usize] += dp[(sum - num) as usize];
                }
            }
        }
        return dp[target as usize];
    }
}
