/// @author: Leon
/// https://leetcode.com/problems/valid-anagram/
/// Time Complexity:    O(`_len_s` + `_len_t`) ~ O(max(`_len_s`, `_len_t`))
/// Space Complexity:   O(26) ~ O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let _len_s = s.len();
        let _len_t = t.len();
        let freqs_s = Self::get_hash(s);
        let freqs_t = Self::get_hash(t);
        for (idx, &freq_s) in freqs_s.iter().enumerate() {
            if freqs_t[idx] != freq_s {
                return false;
            }
        }
        true
    }
    fn get_hash(s: String) -> Vec<u16> {
        let freqs: Vec<u16> = {
            let mut freqs: Vec<u16> = vec![0; 26];
            for ch in s.chars() {
                let idx = ch as usize - 'a' as usize;
                freqs[idx] += 1;
            }
            freqs
        };
        freqs
    }
}
