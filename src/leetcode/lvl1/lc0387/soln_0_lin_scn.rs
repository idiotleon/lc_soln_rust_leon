/// @author: Leon
/// https://leetcode.com/problems/first-unique-character-in-a-string/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(26) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let _len_s: usize = s.len();
        let freqs: Vec<u32> = {
            let mut freqs: Vec<u32> = vec![0; 26];
            for ch in s.chars() {
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        for (idx, ch) in s.chars().enumerate() {
            if freqs[ch as usize - 'a' as usize] == 1 {
                return idx as i32;
            }
        }
        -1
    }
}
