/// @author: Leon
/// https://leetcode.com/problems/longest-ideal-subsequence/
/// Time Complexity:    O(`len_s` ^ 2)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut dp: Vec<i32> = vec![1; len_s];
        let mut longest: i32 = 1;
        for hi in 1..len_s {
            for lo in 0..hi {
                if (chs[lo] as i32 - chs[hi] as i32).abs() <= k {
                    dp[hi] = std::cmp::max(dp[hi], dp[lo] + 1);
                }
            }
            longest = std::cmp::max(longest, dp[hi]);
        }
        longest
    }
}
