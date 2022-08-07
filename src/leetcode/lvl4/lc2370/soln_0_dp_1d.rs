/// @author: Leon
/// https://leetcode.com/problems/longest-ideal-subsequence/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/longest-ideal-subsequence/discuss/2390512/JavaC%2B%2BPython-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let k: usize = k as usize;
        let _len_s: usize = s.len();
        let mut longest: i32 = 0;
        let mut dp: Vec<i32> = vec![0; 150];
        for ch in s.chars() {
            let mut len: i32 = 0;
            let idx_ch: usize = ch as usize;
            for idx2 in (idx_ch - k)..=(idx_ch + k) {
                len = std::cmp::max(len, dp[idx2] + 1);
            }
            dp[idx_ch] = len;
            longest = std::cmp::max(longest, len);
        }
        return longest;
    }
}
