/// https://leetcode.com/problems/valid-palindrome-iii/
///
/// Time Complexity:    O(`len_s` ^ 2)
/// Space Complexity:   O(`len_s` ^ 2)
///
/// Reference:
/// https://leetcode.com/problems/valid-palindrome-iii/discuss/397606/Find-Longest-Palindromic-Subsequence./358118
use std::cmp::max;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_palindrome(s: String, k: i32) -> bool {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![0; len_s + 1]; len_s + 1];

        for d in 0..len_s {
            for lo in 1..=len_s - d {
                let hi = lo + d;

                dp[lo][hi] = if lo == hi {
                    1
                } else if chs[lo - 1] == chs[hi - 1] {
                    dp[lo + 1][hi - 1] + 2
                } else {
                    max(dp[lo + 1][hi], dp[lo][hi - 1])
                }
            }
        }
        len_s - dp[1][len_s] <= k as usize
    }
}
