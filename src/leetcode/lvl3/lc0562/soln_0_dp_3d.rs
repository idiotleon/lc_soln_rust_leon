use std::cmp::max;

/// @author: Leon
/// https://leetcode.com/problems/longest-line-of-consecutive-one-in-matrix/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(`len_r` * `len_c`)
/// Reference:
/// https://leetcode.com/problems/longest-line-of-consecutive-one-in-matrix/discuss/102266/Java-O(nm)-Time-DP-Solution/152499
/// https://leetcode.com/problems/longest-line-of-consecutive-one-in-matrix/discuss/102266/Java-O(nm)-Time-DP-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_line(mat: Vec<Vec<i32>>) -> i32 {
        let len_r: usize = mat.len();
        let len_c: usize = mat[0].len();
        const ONE: i32 = 1;
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 4]; len_c + 2]; len_r + 2];
        let mut longest: i32 = 0;
        for r in 0..len_r {
            for c in 0..len_c {
                if mat[r][c] == ONE {
                    dp[r + 1][c + 1][0] = 1 + dp[r + 1][c][0];
                    dp[r + 1][c + 1][1] = 1 + dp[r][c + 1][1];
                    dp[r + 1][c + 1][2] = 1 + dp[r][c][2];
                    dp[r + 1][c + 1][3] = 1 + dp[r][c + 2][3];
                    for len in dp[r + 1][c + 1].iter() {
                        longest = max(longest, *len);
                    }
                }
            }
        }
        longest
    }
}
