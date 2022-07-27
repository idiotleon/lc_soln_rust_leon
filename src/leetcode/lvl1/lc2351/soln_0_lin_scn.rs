/// @author: Leon
/// https://leetcode.com/problems/first-letter-to-appear-twice/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let _len_s: usize = s.len();
        let mut freqs: Vec<u8> = vec![0; 26];
        for ch in s.chars() {
            let idx: usize = ch as usize - 'a' as usize;
            let freq = &mut freqs[idx];
            *freq += 1;
            if *freq == 2 {
                return ch;
            }
        }
        unreachable!();
    }
}
