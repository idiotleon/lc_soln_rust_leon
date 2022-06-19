/// @author: Leon
/// https://leetcode.com/problems/greatest-english-letter-in-upper-and-lower-case/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let _len_s: usize = s.len();
        const IMPL: char = '#';
        let (freqs_lower, freqs_upper): (Vec<u16>, Vec<u16>) = {
            let mut freqs_lower: Vec<u16> = vec![0; 26];
            let mut freqs_upper: Vec<u16> = vec![0; 26];
            for ch in s.chars() {
                if ch.is_lowercase() {
                    freqs_lower[ch as usize - 'a' as usize] += 1;
                } else {
                    freqs_upper[ch as usize - 'A' as usize] += 1;
                }
            }
            (freqs_lower, freqs_upper)
        };
        let mut ans: char = IMPL;
        for (idx, freq_lower) in freqs_lower.into_iter().enumerate() {
            if freq_lower > 0 && freqs_upper[idx] > 0 {
                ans = ('A' as u8 + idx as u8) as char;
            }
        }
        if ans == IMPL {
            "".to_owned()
        } else {
            ans.to_string()
        }
    }
}
