/// @author: Leon
/// https://leetcode.com/problems/4-keys-keyboard/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_a(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; 7];
        for i in 1..=n {
            dp[0] = i;
            for k in (3..7).rev() {
                dp[0] = std::cmp::max(dp[0], dp[k] * (k as i32 - 1));
            }
            for k in (1..7).rev() {
                dp[k] = dp[k - 1];
            }
        }
        return dp[0];
    }
}
