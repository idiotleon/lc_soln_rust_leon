/// @author: Leon
/// https://leetcode.com/problems/unique-paths/
/// Time Complexity:     O(`m` * `n`)
/// Space Complexity:    O(`n`)
/// Reference:
/// https://leetcode.com/problems/unique-paths/discuss/22954/C%2B%2B-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m: usize = m as usize;
        let n: usize = n as usize;
        let mut dp: Vec<Vec<i32>> = {
            let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];
            for r in 0..m {
                dp[r][0] = 1;
            }
            for c in 0..n {
                dp[0][c] = 1;
            }
            dp
        };
        for r in 1..m {
            for c in 1..n {
                dp[r][c] = dp[r - 1][c] + dp[r][c - 1];
            }
        }
        dp[m - 1][n - 1]
    }
}
