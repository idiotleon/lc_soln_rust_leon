/// https://leetcode.com/problems/longest-palindromic-substring/
///
/// Time Complexity:     O(`len_s` ^ 2)
/// Space Complexity:    O(`len_s`)
///
/// Reference:
/// https://leetcode.com/problems/longest-palindromic-substring/discuss/295110/Rust-DP-Solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len_s = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; len_s]; len_s];
        let mut idx_start = 0;
        let mut idx_end = 0;

        for lo in (0..len_s).rev() {
            for hi in lo..len_s {
                dp[lo][hi] = chs[lo] == chs[hi] && (hi - lo < 3 || dp[lo + 1][hi - 1]);
                if dp[lo][hi] {
                    if hi - lo + 1 > idx_end - idx_start {
                        idx_start = lo;
                        idx_end = hi + 1;
                    }
                }
            }
        }
        chs[idx_start..idx_end].into_iter().collect::<String>()
    }
}
