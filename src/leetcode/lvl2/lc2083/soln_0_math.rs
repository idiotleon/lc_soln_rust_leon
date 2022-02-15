/// @author: Leon
/// https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/discuss/1613061/Java-7-line-Solution-or-O(n)-Time-or-Easy-To-Understand
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_substrings(s: String) -> i64 {
        let mut freqs: Vec<i64> = vec![0; 26];
        let mut ans: i64 = 0;
        for ch in s.chars() {
            let idx_ch: usize = ch as usize - 'a' as usize;
            freqs[idx_ch] += 1;
            ans += freqs[idx_ch];
        }
        ans
    }
}
