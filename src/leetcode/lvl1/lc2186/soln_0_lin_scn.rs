/// @author: Leon
/// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram-ii/
/// Time Complexity:    O(max(`_len_s`, `_len_t`))
/// Space Complexity:   O(max(`_len_s`, `_len_t`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let _len_s: usize = s.len();
        let _len_t: usize = t.len();
        let hash_s: Vec<i32> = Self::get_hash(s);
        let hash_t: Vec<i32> = Self::get_hash(t);
        let mut cnt_diff: i32 = 0;
        for (idx, freq) in hash_s.into_iter().enumerate() {
            cnt_diff += (freq - hash_t[idx]).abs();
        }
        cnt_diff
    }
    fn get_hash(s: String) -> Vec<i32> {
        let mut freqs: Vec<i32> = vec![0; 26];
        for ch in s.chars() {
            freqs[ch as usize - 'a' as usize] += 1;
        }
        freqs
    }
}
