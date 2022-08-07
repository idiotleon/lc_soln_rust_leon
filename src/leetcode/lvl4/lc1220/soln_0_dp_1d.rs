/// @author: Leon
/// https://leetcode.com/problems/count-vowels-permutation/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let n: usize = n as usize;
        const MOD: i64 = 1e9 as i64 + 7;
        let mut dp: Vec<Vec<i64>> = {
            let mut dp: Vec<Vec<i64>> = vec![vec![0; 5]; n + 1];
            for c in 0..5 {
                dp[1][c] = 1;
            }
            dp
        };
        for r in 1..n {
            dp[1 + r][0] = (dp[r][4] + dp[r][1] + dp[r][2]) % MOD;
            dp[1 + r][1] = (dp[r][0] + dp[r][2]) % MOD;
            dp[1 + r][2] = (dp[r][3] + dp[r][1]) % MOD;
            dp[1 + r][3] = dp[r][2] % MOD;
            dp[1 + r][4] = (dp[r][2] + dp[r][3]) % MOD;
        }
        return (dp[n].iter().sum::<i64>() % MOD) as i32;
    }
}
