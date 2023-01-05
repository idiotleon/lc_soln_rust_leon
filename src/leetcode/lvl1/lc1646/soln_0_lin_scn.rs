/// @author: Leon
/// https://leetcode.com/problems/get-maximum-in-generated-array/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut dp: Vec<i32> = vec![0; n as usize + 1];
        dp[0] = 0;
        dp[1] = 1;
        let mut max: i32 = 1;
        let mut idx: usize = 1;
        while idx * 2 + 1 <= n as usize {
            dp[idx * 2] = dp[idx];
            // max = std::cmp::max(max, dp[idx * 2]);
            dp[idx * 2 + 1] = dp[idx] + dp[idx + 1];
            max = std::cmp::max(max, dp[idx * 2 + 1]);
            idx += 1;
        }
        return max;
    }
}
