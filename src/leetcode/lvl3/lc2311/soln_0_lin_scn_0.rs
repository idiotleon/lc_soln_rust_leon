/// @author: Leon
/// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/discuss/2168227/JavaC%2B%2BPython-One-Pass-O(n)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_subsequence(s: String, mut k: i32) -> i32 {
        let _len_s: usize = s.len();
        let mut len: i32 = 0;
        let mut cost: i32 = 1;
        for &b in s.as_bytes().iter().rev() {
            if b == b'0' || cost <= k {
                k -= cost * (b - b'0') as i32;
                len += 1;
            }
            if cost <= k {
                cost *= 2;
            }
        }
        len
    }
}
