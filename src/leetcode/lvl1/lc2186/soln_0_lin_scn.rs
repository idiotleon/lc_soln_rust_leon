/// @author: Leon
/// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram-ii/
/// Time Complexity:    O(max(`_len_s`, `_len_t`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let freqs_s: Vec<u32> = {
            let mut freqs: Vec<u32> = vec![0; 26];
            for ch in s.chars(){
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        let freqs_t: Vec<u32> = {
            let mut freqs: Vec<u32> = vec![0; 26];
            for ch in t.chars(){
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        let mut cnt: i32 = 0;
        for idx in 0..26{
            cnt += (freqs_s[idx] as i32 - freqs_t[idx] as i32).abs();
        }
        cnt
    }
}