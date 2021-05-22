/// https://leetcode.com/problems/longest-palindromic-subsequence/
///
/// Time Complexity:    O(`len_s` ^ 2)
/// Space Complexity:   O(`len_s` ^ 2)
///
/// Reference:
/// https://leetcode.com/problems/longest-palindromic-subsequence/discuss/99101/Straight-forward-Java-DP-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let len_s = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len_s]; len_s];

        for lo in (0..len_s).rev() {
            dp[lo][lo] = 1;
            for hi in lo + 1..len_s {
                dp[lo][hi] = if chs[lo] == chs[hi] {
                    dp[lo + 1][hi - 1] + 2
                } else {
                    std::cmp::max(dp[lo + 1][hi], dp[lo][hi - 1])
                }
            }
        }
        dp[0][len_s - 1]
    }
}
