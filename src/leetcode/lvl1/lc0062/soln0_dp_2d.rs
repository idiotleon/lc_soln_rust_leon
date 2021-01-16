impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        
        let mut dp = vec![vec![0; n]; m];
        for r in 0..m {
            dp[r][0] = 1;
        }
        for c in 0..n {
            dp[0][c] = 1;
        }
        
        for r in 1..m{
            for c in 1..n{
                dp[r][c] = dp[r - 1][c] + dp[r][c - 1];
            }
        }
        
        return dp[m - 1][n - 1];
    }
}