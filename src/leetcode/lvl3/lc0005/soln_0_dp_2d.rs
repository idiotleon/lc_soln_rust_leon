/// https://leetcode.com/problems/longest-palindromic-substring/
///
/// Time Complexity:     O()
/// Space Complexity:    O()
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut idx_start = 0;
        let mut idx_end = 0;
        let chs: Vec<char> = s.chars().collect();
        let len_chs: usize = chs.len();
        let mut dp = vec![vec![false; chs.len()]; chs.len()];

        for lo in (0..len_chs).rev() {
            for hi in lo..len_chs {
                dp[lo][hi] = (chs[lo] == chs[hi]) && (hi - lo < 2 || dp[lo + 1][hi - 1]);

                if dp[lo][hi] && (hi - lo > idx_end - idx_start) {
                    idx_start = lo;
                    idx_end = hi;
                }
            }
        }
        chs[idx_start..idx_end + 1].iter().collect::<String>()
    }
}
