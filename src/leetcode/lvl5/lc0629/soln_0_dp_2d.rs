/// @author: Leon
/// https://leetcode.com/problems/k-inverse-pairs-array/
/// Time Complexity:    O(`n` * `k`)
/// Space Complexity:   O(`n` * `k`)
/// Reference:
/// https://leetcode.com/problems/k-inverse-pairs-array/discuss/104815/Java-DP-O(nk)-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MOD: u64 = 1e9 as u64 + 7;
        if k > n * (n - 1) / 2 || k < 0 {
            return 0;
        }
        if k == 0 || k == n * (n - 1) / 2 {
            return 1;
        }
        let n: usize = n as usize;
        let k: usize = k as usize;
        let mut dp: Vec<Vec<u64>> = {
            let mut dp: Vec<Vec<u64>> = vec![vec![0; k + 1]; n + 1];
            dp[2][0] = 1;
            dp[2][1] = 1;
            dp
        };
        for idx_n in 3..=n {
            dp[idx_n][0] = 1;
            for idx_k in 1..=std::cmp::min(k, idx_n * (idx_n - 1) / 2) {
                dp[idx_n][idx_k] = dp[idx_n][idx_k - 1] + dp[idx_n - 1][idx_k];
                if idx_k >= idx_n {
                    dp[idx_n][idx_k] -= dp[idx_n - 1][idx_k - idx_n];
                }
                dp[idx_n][idx_k] = (dp[idx_n][idx_k] + MOD) % MOD;
            }
        }
        dp[n][k] as i32
    }
}
