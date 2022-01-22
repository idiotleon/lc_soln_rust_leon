/// @author: Leon
/// https://leetcode.com/problems/find-the-difference/
/// Time Complexity:    O(`len_s` + `len_t`) ~ O(`len_t`)
/// Space Complexity:   O(26) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let _len_s: usize = s.len();
        let _len_t: usize = t.len();
        let mut freqs: Vec<i16> = vec![0; 26];
        for ch in s.chars() {
            freqs[ch as usize - 'a' as usize] += 1;
        }
        for ch in t.chars() {
            let idx = ch as usize - 'a' as usize;
            freqs[idx] -= 1;
            if freqs[idx] < 0 {
                return ch;
            }
        }
        unreachable!()
    }
}
