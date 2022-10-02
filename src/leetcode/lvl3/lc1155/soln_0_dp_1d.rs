/// @author: Leon
/// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/description/
/// Time Complexity:    O(`n` * `k` * `target`)
/// Space Complexity:   O(`target`)
/// Reference:
/// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/solutions/355940/c-coin-change-2/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        let mut dp: Vec<i32> = vec![0; target as usize + 1];
        dp[0] = 1;
        for _dice in 1..=n {
            let mut dp1 = vec![0; target as usize + 1];
            for face in 1..=k {
                for t in face..=target {
                    dp1[t as usize] = (dp1[t as usize] + dp[(t - face) as usize]) % MOD;
                }
            }
            dp = dp1;
        }
        return dp[target as usize];
    }
}
