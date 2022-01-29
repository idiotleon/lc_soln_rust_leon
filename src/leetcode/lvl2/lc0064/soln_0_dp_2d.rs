/// @author: Leon
/// https://leetcode.com/problems/minimum-path-sum/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(`len_r` * `len_c`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len_c]; len_r];
        dp[0][0] = grid[0][0];
        for r in 1..len_r {
            dp[r][0] = dp[r - 1][0] + grid[r][0];
        }
        for c in 1..len_c {
            dp[0][c] = dp[0][c - 1] + grid[0][c];
        }
        for r in 1..len_r {
            for c in 1..len_c {
                dp[r][c] = grid[r][c] + std::cmp::min(dp[r - 1][c], dp[r][c - 1]);
            }
        }
        dp[len_r - 1][len_c - 1]
    }
}
