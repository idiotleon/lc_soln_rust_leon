/// @author: Leon
/// https://leetcode.com/problems/reverse-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len_s: usize = s.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_s - 1;
        while lo < hi {
            s.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
    }
}
