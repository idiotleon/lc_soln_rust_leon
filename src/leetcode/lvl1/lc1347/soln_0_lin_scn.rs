/// @author: Leon
/// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/
/// Time Complexity:    O(`len_s`) / O(`len_t`)
/// Space Complexity:   O(`len_s`) / O(`len_t`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let len_s: usize = s.len();
        let len_t: usize = t.len();
        // sanity check
        if len_s != len_t {
            return -1;
        }
        let freqs_s: Vec<i32> = Self::get_hash(s);
        let freqs_t: Vec<i32> = Self::get_hash(t);
        let mut cnt_diff: i32 = 0;
        for (idx, freq) in freqs_s.into_iter().enumerate() {
            cnt_diff += (freq - freqs_t[idx]).abs();
        }
        cnt_diff / 2
    }
    fn get_hash(s: String) -> Vec<i32> {
        let mut freqs: Vec<i32> = vec![0; 26];
        for ch in s.chars() {
            freqs[ch as usize - 'a' as usize] += 1;
        }
        freqs
    }
}
