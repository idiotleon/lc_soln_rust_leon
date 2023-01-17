/// @author: Leon
/// https://leetcode.com/problems/flip-string-to-monotone-increasing/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://zxi.mytechroad.com/blog/dynamic-programming/leetcode-926-flip-string-to-monotone-increasing/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        const ONE: char = '1';
        const ZERO: char = '0';
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; len_s + 1];
        for idx in 1..=len_s {
            match chs[idx - 1] {
                ZERO => {
                    dp[idx][0] = dp[idx - 1][0];
                    dp[idx][1] = std::cmp::min(dp[idx - 1][0], dp[idx - 1][1]) + 1;
                }
                ONE => {
                    dp[idx][0] = dp[idx - 1][0] + 1;
                    dp[idx][1] = std::cmp::min(dp[idx - 1][0], dp[idx - 1][1]);
                }
                _ => unreachable!(),
            }
        }
        return std::cmp::min(dp[len_s][0], dp[len_s][1]);
    }
}
