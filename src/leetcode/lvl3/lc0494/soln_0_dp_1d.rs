/// @author: Leon
/// https://leetcode.com/problems/target-sum/
/// Time Complexity:    O(`_len_ns` * `sum_total`)
/// Space Complexity:   O(`range`) ~ O(`sum_total`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let _len_ns: usize = nums.len();
        let sum_total: i32 = nums.iter().sum();
        if target > sum_total || target < -sum_total {
            return 0;
        }
        let range: i32 = 2 * sum_total + 1;
        let mut dp: Vec<i32> = {
            let mut dp = vec![0; range as usize];
            dp[0 + sum_total as usize] = 1;
            dp
        };
        for num in nums {
            let mut dp_nxt: Vec<i32> = vec![0; range as usize];
            for sum in 0..range {
                if dp[sum as usize] == 0 {
                    continue;
                }
                dp_nxt[(sum + num) as usize] += dp[sum as usize];
                dp_nxt[(sum - num) as usize] += dp[sum as usize];
            }
            dp = dp_nxt;
        }
        return dp[(sum_total + target) as usize];
    }
}
