/// @author: Leon
/// https://leetcode.com/problems/target-sum/
/// Time Complexity:    O(`len_ns` * `sum_total`)
/// Space Complexity:   O(`len_ns` * `sum_total`)
/// Reference:
/// https://leetcode.com/problems/target-sum/discuss/97335/Short-Java-DP-Solution-with-Explanation/101899
/// https://leetcode.com/problems/target-sum/discuss/97335/Short-Java-DP-Solution-with-Explanation/239358
/// https://leetcode.com/problems/target-sum/discuss/97335/Short-Java-DP-Solution-with-Explanation/220467
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let len_ns: usize = nums.len();
        let sum_total: i32 = nums.iter().sum();
        if target > sum_total || target < -sum_total {
            return 0;
        }
        let range: i32 = 2 * sum_total + 1;
        let mut dp: Vec<Vec<i32>> = {
            let mut dp: Vec<Vec<i32>> = vec![vec![0; range as usize]; len_ns + 1];
            dp[0][sum_total as usize] = 1;
            dp
        };
        for idx in 1..=len_ns {
            for sum in 0..range {
                let sum_plus: i32 = sum + nums[idx - 1];
                if sum_plus < range {
                    dp[idx][sum as usize] += dp[idx - 1][sum_plus as usize];
                }
                let sum_minus: i32 = sum - nums[idx - 1];
                if sum_minus >= 0 {
                    dp[idx][sum as usize] += dp[idx - 1][sum_minus as usize];
                }
            }
        }
        return dp[len_ns][sum_total as usize + target as usize];
    }
}
